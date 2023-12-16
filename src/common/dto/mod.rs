use std::string::ToString;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// rest 请求头
#[derive(Deserialize, Serialize, Debug,Clone)]
#[serde(deny_unknown_fields)]
pub struct RestRequest<T> {
    locale: Locale,
    request: T,
}
#[derive(Debug, Deserialize, Serialize,Clone)]
pub enum Locale {
    ZH_CN,
    ZH_TW,
    ZH_HK,
    EN_US,
}

/// rest响应体
#[derive(Serialize,Debug)]
pub struct RestResponse<T>
{
    pub response: T,
    pub trace_id: String,
}
impl<T> RestResponse<T> {
    pub fn new(data: T) -> RestResponse<T> {
        RestResponse {
            trace_id: Uuid::new_v4().to_string(),
            response: data,
        }
    }
    pub fn trace_id(&self) -> &str{
        self.trace_id.as_str()
    }
}
