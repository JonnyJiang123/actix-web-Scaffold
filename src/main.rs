use actix_web_Scaffold::app;
use std::env;
use tracing;
use tracing_subscriber as ts;
use opentelemetry::global;
use tracing_subscriber::{
    fmt,layer::SubscriberExt,util::SubscriberInitExt
};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE","full");
    env::set_var("RUST_LOG", "info");

    global::set_text_map_propagator(opentelemetry_jaeger::Propagator::new());
    let tracer = opentelemetry_jaeger::new_pipeline().with_service_name("scaffold").install_simple().expect("pipeline error");
    let opentelemetry = tracing_opentelemetry::layer().with_tracer(tracer);
    ts::registry()
        .with(opentelemetry)
        .with(fmt::Layer::default())
        .try_init()
        .expect("registry error");

    app::run().await
}