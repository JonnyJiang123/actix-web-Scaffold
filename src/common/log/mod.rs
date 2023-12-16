use serde::Serialize;
use tracing::{event, Span,Level};
use crate::common::dto::{RestRequest, RestResponse};

pub fn fill_trace_args<T:Serialize,R:Serialize>(request:&RestRequest<T>, response:&RestResponse<R>) {
    let request_json = serde_json::json!(request);
    let response_json = serde_json::json!(response);
    Span::current().record("trace_id", response.trace_id());
    Span::current().record("request", request_json.to_string());
    Span::current().record("response", response_json.to_string());
}
pub fn info(msg:String){
    event!(Level::INFO,msg);
}
pub fn err(msg:String){
    event!(Level::ERROR,msg);
}