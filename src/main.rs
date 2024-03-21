use anyhow::Result;
use dotenvy::var;
use opentelemetry::global::{set_text_map_propagator, shutdown_tracer_provider};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::propagation::TraceContextPropagator;
use tracing::{instrument, Instrument};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn configure_tracing(level: String) -> Result<()> {
    set_text_map_propagator(TraceContextPropagator::new());
    let exporter = opentelemetry_otlp::new_exporter().tonic().with_endpoint(
        var("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT")
            .unwrap_or_else(|_| "http://127.0.0.1:4317".to_string()),
    );
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(opentelemetry_sdk::trace::config().with_resource(
            opentelemetry_sdk::Resource::new(vec![opentelemetry::KeyValue::new(
                "service.name",
                "some_service",
            )]),
        ))
        .install_simple()?;

    let tracer = tracing_opentelemetry::layer().with_tracer(tracer);

    let level = EnvFilter::new(level);

    tracing_subscriber::registry()
        .with(level)
        .with(tracer)
        .init();

    Ok(())
}

#[instrument]
pub async fn my_function() {}

#[instrument]
pub async fn some_function() {
    my_function().await;
    another_function().await;
}

#[instrument]
pub async fn another_function() {
    my_function().await;
}

#[tokio::main]
async fn main() -> Result<()> {
    configure_tracing("info".to_owned())?;

    my_function().await;

    let span = tracing::info_span!("hei");
    async move {
        my_function().await;
        some_function().await;
    }
    .instrument(span)
    .await;

    some_function().await;

    shutdown_tracer_provider();
    Ok(())
}
