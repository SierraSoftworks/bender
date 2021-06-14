use opentelemetry::{KeyValue, sdk};
use tracing::metadata::LevelFilter;
use tracing_honeycomb::new_honeycomb_telemetry_layer;
use tracing_subscriber::{Registry, prelude::__tracing_subscriber_SubscriberExt};

pub struct Session {
}

impl Session {
    pub fn new() -> Self {
        let honeycomb_key = std::env::var("HONEYCOMB_KEY").unwrap_or_default();
        let app_insights_key = std::env::var("APPINSIGHTS_INSTRUMENTATIONKEY").unwrap_or_default();

        if !honeycomb_key.is_empty() {
            let config = libhoney::Config {
                options: libhoney::client::Options {
                    api_key: honeycomb_key,
                    dataset: "bender.sierrasoftworks.com".to_string(),
                    ..Default::default()
                },
                transmission_options: libhoney::transmission::Options::default(),
            };

            let telemetry = new_honeycomb_telemetry_layer("Bender", config);
            let subscriber = Registry::default()
                .with(LevelFilter::INFO)
                .with(tracing_subscriber::fmt::Layer::default())
                .with(telemetry);
            tracing::subscriber::set_global_default(subscriber).unwrap_or_default();
    
            Self {
            }
        } else if !app_insights_key.is_empty() {
            let tracer = opentelemetry_application_insights::new_pipeline(
                app_insights_key
            )
                .with_client(reqwest::Client::new())
                .with_trace_config(sdk::trace::config().with_resource(sdk::Resource::new(vec![
                    KeyValue::new("service.name", "Bender"),
                    KeyValue::new("service.version", env!("CARGO_PKG_VERSION"))
                ])))
                .install_batch(opentelemetry::runtime::Tokio);
        
            let telemetry = tracing_opentelemetry::layer()
                .with_tracked_inactivity(true)
                .with_tracer(tracer);
        
            let subscriber = Registry::default().with(telemetry);
            tracing::subscriber::set_global_default(subscriber).unwrap_or_default();
    
            Self {
            }
        } else {
            
            let tracer = sdk::export::trace::stdout::new_pipeline()
                .install_simple();

            let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);
            let subscriber = Registry::default().with(telemetry);
            
            tracing::subscriber::set_global_default(subscriber).unwrap_or_default();

            // tracing_subscriber::fmt()
            //     .with_max_level(tracing::Level::INFO)
            //     .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
            //     .init();

            Self {
            }
        }
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        opentelemetry::global::shutdown_tracer_provider();
    }
}