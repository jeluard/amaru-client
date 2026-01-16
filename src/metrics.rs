use axum::{
    Router,
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
};
use futures::{Stream, StreamExt};
use opentelemetry_proto::tonic::{
    collector::metrics::v1::ExportMetricsServiceRequest, metrics::v1::Metric,
};
use prost::Message;
use std::{net::SocketAddr, pin::Pin, sync::Arc};
use tokio::{
    net::TcpListener,
    sync::broadcast::{self, Sender},
};
use tokio_stream::wrappers::BroadcastStream;

pub struct AmaruMetricsService {
    tx: Sender<Metric>,
}

async fn handle_metrics(
    State(tx): State<Arc<Sender<Metric>>>,
    _headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    match ExportMetricsServiceRequest::decode(body.as_ref()) {
        Ok(req) => {
            for resource_metrics in req.resource_metrics {
                for scope_metrics in resource_metrics.scope_metrics {
                    for metric in scope_metrics.metrics {
                        if let Err(err) = tx.send(metric) {
                            eprintln!("broadcast send error: {}", err);
                        }
                    }
                }
            }
            StatusCode::OK
        }
        Err(_) => StatusCode::BAD_REQUEST,
    }
}

pub type MetricStream = Pin<Box<dyn Stream<Item = Metric> + Send>>;

impl AmaruMetricsService {
    pub fn new(buffer: usize) -> Self {
        let (tx, _) = broadcast::channel(buffer);
        Self { tx }
    }

    pub fn subscribe(&self) -> MetricStream {
        let rx = self.tx.subscribe();
        Box::pin(BroadcastStream::new(rx).filter_map(|r| futures::future::ready(r.ok())))
    }

    pub async fn start(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        let app = Router::new()
            .route("/v1/metrics", post(handle_metrics))
            .with_state(Arc::new(self.tx));
        axum::serve(TcpListener::bind(addr).await.unwrap(), app).await?;

        Ok(())
    }
}
