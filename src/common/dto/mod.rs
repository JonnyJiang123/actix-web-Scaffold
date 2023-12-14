use serde::{Deserialize, Serialize};
use uuid::Uuid;
/// rest 请求头
#[derive(Deserialize, Serialize, Debug)]
pub struct RestRequest<T> {
    locale: Locale,
    request: T,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Locale {
    ZH_CN,
    ZH_TW,
    ZH_HK,
    EN_US,
}

/// rest响应体
#[derive(Serialize)]
pub struct RestResponse<T> {
    response: T,
    trace_id: String,
}
impl<T> RestResponse<T> {
    pub fn new(data: T) -> RestResponse<T> {
        RestResponse {
            trace_id: Uuid::new_v4().to_string(),
            response: data,
        }
    }
}
