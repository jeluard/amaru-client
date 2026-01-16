use futures::stream::{Stream, StreamExt};
use opentelemetry_proto::tonic::{
    collector::trace::v1::{
        ExportTraceServiceRequest, ExportTraceServiceResponse,
        trace_service_server::{TraceService, TraceServiceServer},
    },
    trace::v1::Span,
};
use std::{net::SocketAddr, pin::Pin};
use tokio::sync::broadcast;
use tokio_stream::wrappers::BroadcastStream;
use tonic::{Request, Response, Status, transport::Server};

pub type SpanStream = Pin<Box<dyn Stream<Item = Span> + Send>>;

pub struct AmaruTracesService {
    tx: broadcast::Sender<Span>,
}

impl AmaruTracesService {
    pub fn new(buffer: usize) -> Self {
        let (tx, _) = broadcast::channel(buffer);
        Self { tx }
    }

    pub fn subscribe(&self) -> SpanStream {
        let rx = self.tx.subscribe();
        Box::pin(BroadcastStream::new(rx).filter_map(|r| futures::future::ready(r.ok())))
    }

    pub async fn start(self, addr: SocketAddr) -> Result<(), Box<dyn std::error::Error>> {
        Server::builder()
            .add_service(TraceServiceServer::new(self))
            .serve(addr)
            .await?;
        Ok(())
    }
}

#[tonic::async_trait]
impl TraceService for AmaruTracesService {
    async fn export(
        &self,
        req: Request<ExportTraceServiceRequest>,
    ) -> Result<Response<ExportTraceServiceResponse>, Status> {
        req.into_inner()
            .resource_spans
            .into_iter()
            .flat_map(|r| r.scope_spans.into_iter())
            .flat_map(|s| s.spans.into_iter())
            .for_each(|span| {
                if let Err(err) = self.tx.send(span) {
                    eprintln!("broadcast send error: {}", err);
                }
            });

        Ok(Response::new(ExportTraceServiceResponse::default()))
    }
}
