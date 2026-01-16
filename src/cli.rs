use clap::{Args, Parser, Subcommand};
use std::{error::Error, net::SocketAddr};
use tokio_stream::StreamExt;

use crate::{metrics::AmaruMetricsService, otel::AmaruTracesService};

#[derive(Args, Debug)]
pub struct MetricsArgs {
    /// Address to bind metrics to
    #[arg(long, default_value = "[::1]:4318")]
    pub address: String,
}

#[derive(Args, Debug)]
pub struct TracesArgs {
    /// Address to bind traces to
    #[arg(long, default_value = "[::1]:4317")]
    pub address: String,
}

#[derive(Subcommand)]
pub enum Commands {
    Metrics(MetricsArgs),
    Traces(TracesArgs),
}

#[derive(Args, Debug)]
pub struct BothArgs {
    #[arg(long, default_value = "[::1]:4318")]
    pub metrics_address: String,

    #[arg(long, default_value = "[::1]:4317")]
    pub traces_address: String,
}

#[derive(Parser)]
#[command(about = "Remote CLI for Amaru")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[command(flatten)]
    pub both: BothArgs,
}

async fn subscribe_traces(address: String) -> Result<(), Box<dyn Error + 'static>> {
    let addr: SocketAddr = address.parse()?;
    let trace_service = AmaruTracesService::new(10);
    let mut stream = trace_service
        .subscribe()
        .map(|span| serde_json::to_string(&span));
    tokio::spawn(async move {
        while let Some(item) = stream.next().await {
            match item {
                Ok(json) => println!("{json}"),
                Err(err) => {
                    eprintln!("json error: {err}");
                    break;
                }
            }
        }
    });

    eprintln!("Traces service listening on {}", addr);

    trace_service.start(addr).await?;

    Ok(())
}

async fn subscribe_metrics(address: String) -> Result<(), Box<dyn Error + 'static>> {
    let addr: SocketAddr = address.parse()?;
    let metric_service = AmaruMetricsService::new(10);
    let mut stream = metric_service
        .subscribe()
        .map(|span| serde_json::to_string(&span));
    tokio::spawn(async move {
        while let Some(item) = stream.next().await {
            match item {
                Ok(json) => println!("{json}"),
                Err(err) => {
                    eprintln!("json error: {err}");
                    break;
                }
            }
        }
    });

    eprintln!("Metrics service listening on {}", addr);

    metric_service.start(addr).await?;

    Ok(())
}

impl Cli {
    pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
        let cli = Self::parse();
        match cli.command {
            Some(Commands::Traces(args)) => {
                subscribe_traces(args.address).await?;
            }
            Some(Commands::Metrics(args)) => {
                subscribe_metrics(args.address).await?;
            }
            None => {
                let (traces_result, metrics_result) = tokio::join!(
                    subscribe_traces(cli.both.traces_address),
                    subscribe_metrics(cli.both.metrics_address),
                );
                traces_result?;
                metrics_result?;
            }
        }

        Ok(())
    }
}
