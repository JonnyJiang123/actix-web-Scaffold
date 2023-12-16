Rust企业web开发脚手架

集成OpenTelemetry
1. docker run -d -p6831:6831/udp -p6832:6832/udp -p16686:16686 -p14268:14268 jaegertracing/all-in-one:latest
2. 添加配置
```toml
# Implements the types defined in the Otel spec
opentelemetry = "0.17.0"
# Integration between the tracing crate and the opentelemetry crate
tracing-opentelemetry = "0.17.2" 
# Allows you to export data to Jaeger
opentelemetry-jaeger = "0.16.0"
```
3. 日志采集配置 
main.rs
注意不能初始化其他等日志了
```rust
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
```
4. 使用
直接先声明变量，后续再填充值。每个接口根据实际情况进行打日志，通过tracing::event!(tracing::Level::INFO,"xxx")
```rust
#[post("/create")]
#[tracing::instrument(name="create",fields(trace_id,request,response),err(level = tracing::Level::ERROR))]
pub async fn create(json: web::Json<RestRequest<User>>) -> Result<impl Responder>{
    let user = json.into_inner();
    let response = RestResponse::new(user.clone());

    fill_trace_args(&user,&response);
    Ok(web::Json(response))
}
fn fill_trace_args<T:Serialize,R:Serialize>(request:&RestRequest<T>,response:&RestResponse<R>){
    let request_json = serde_json::json!(request);
    let response_json = serde_json::json!(response);
    Span::current().record("trace_id",response.trace_id());
    Span::current().record("request",request_json.to_string());
    Span::current().record("response",response_json.to_string());
}
```
