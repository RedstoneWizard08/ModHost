//! Utilities for bootstrapping a logger.

use std::time::Duration;

use crate::Result;
use opentelemetry::{KeyValue, global, trace::TracerProvider as OTracerProvider};
use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, MetricExporter, SpanExporter};
use opentelemetry_sdk::{
    Resource,
    logs::LoggerProvider,
    metrics::{MeterProviderBuilder, PeriodicReader, SdkMeterProvider, Temporality},
    runtime,
    trace::{RandomIdGenerator, Sampler, TracerProvider},
};
use tracing_opentelemetry::{MetricsLayer, OpenTelemetryLayer};
use tracing_subscriber::{
    EnvFilter, Layer, filter::LevelFilter, fmt, layer::SubscriberExt, registry,
    util::SubscriberInitExt,
};

/// Converts a [`log::LevelFilter`] to a [`LevelFilter`].
pub fn from_log_level(level: log::LevelFilter) -> LevelFilter {
    match level {
        log::LevelFilter::Debug => LevelFilter::DEBUG,
        log::LevelFilter::Error => LevelFilter::ERROR,
        log::LevelFilter::Info => LevelFilter::INFO,
        log::LevelFilter::Off => LevelFilter::OFF,
        log::LevelFilter::Trace => LevelFilter::TRACE,
        log::LevelFilter::Warn => LevelFilter::WARN,
    }
}

/// Construct the MeterProvider for MetricsLayer
pub fn init_meter_provider(resource: Resource) -> SdkMeterProvider {
    let exporter = MetricExporter::builder()
        .with_tonic()
        .with_temporality(Temporality::default())
        .build()
        .unwrap();

    let reader = PeriodicReader::builder(exporter, runtime::Tokio)
        .with_interval(Duration::from_secs(30))
        .build();

    let meter_provider = MeterProviderBuilder::default()
        .with_resource(resource)
        .with_reader(reader)
        .build();

    global::set_meter_provider(meter_provider.clone());

    meter_provider
}

/// Construct the TracerProvider for OpenTelemetryLayer
pub fn init_tracer_provider(resource: Resource) -> TracerProvider {
    let exporter = SpanExporter::builder().with_tonic().build().unwrap();

    TracerProvider::builder()
        // Customize sampling strategy
        .with_sampler(Sampler::ParentBased(Box::new(Sampler::TraceIdRatioBased(
            1.0,
        ))))
        // If export trace to AWS X-Ray, you can use XrayIdGenerator
        .with_id_generator(RandomIdGenerator::default())
        .with_resource(resource)
        .with_batch_exporter(exporter, runtime::Tokio)
        .build()
}

/// A structure holding the OpenTelemetry providers that need to be dropped.
pub struct OtelGuard {
    tracer_provider: TracerProvider,
    meter_provider: SdkMeterProvider,
    logger_provider: LoggerProvider,
}

/// Initializes the file logger.
pub fn init_logger(service_name: impl AsRef<str>, verbosity: LevelFilter) -> Result<OtelGuard> {
    let mut filter = EnvFilter::from_default_env().add_directive(verbosity.into());

    filter = filter.add_directive("tokio_postgres::connection=warn".parse().unwrap());
    filter = filter.add_directive("tokio_postgres::query=warn".parse().unwrap());
    filter = filter.add_directive("diesel_async_migrations=warn".parse().unwrap());
    filter = filter.add_directive("tokio_util::codec::framed_impl=warn".parse().unwrap());
    filter = filter.add_directive("tokio_tungstenite=warn".parse().unwrap());
    filter = filter.add_directive("want=warn".parse().unwrap());
    filter = filter.add_directive("tungstenite=warn".parse().unwrap());
    filter = filter.add_directive("arboard=warn".parse().unwrap());

    let log_exporter = LogExporter::builder().with_tonic().build()?;

    let resource = Resource::new(vec![KeyValue::new(
        "service.name",
        service_name.as_ref().to_string(),
    )]);

    let logger_provider = LoggerProvider::builder()
        .with_batch_exporter(log_exporter, runtime::Tokio)
        .with_resource(resource.clone())
        .build();

    let tracer_provider = init_tracer_provider(resource.clone());
    let meter_provider = init_meter_provider(resource);
    let tracer = tracer_provider.tracer("tracing-otel-subscriber");
    let otel_log_layer = OpenTelemetryTracingBridge::new(&logger_provider);
    let otel_trace_layer = OpenTelemetryLayer::new(tracer);
    let otel_metrics_layer = MetricsLayer::new(meter_provider.clone());

    let layer = fmt::layer()
        .pretty()
        .compact()
        .with_ansi(true)
        .with_level(true)
        .with_target(true)
        .with_file(false)
        .with_line_number(false)
        .without_time();

    registry()
        .with(otel_metrics_layer)
        .with(otel_trace_layer)
        .with(otel_log_layer)
        .with(layer.with_filter(filter))
        .init();

    Ok(OtelGuard {
        logger_provider,
        meter_provider,
        tracer_provider,
    })
}

impl Drop for OtelGuard {
    fn drop(&mut self) {
        if let Err(err) = self.tracer_provider.shutdown() {
            eprintln!("{err:?}");
        }

        if let Err(err) = self.meter_provider.shutdown() {
            eprintln!("{err:?}");
        }

        if let Err(err) = self.logger_provider.shutdown() {
            eprintln!("{err:?}");
        }
    }
}
