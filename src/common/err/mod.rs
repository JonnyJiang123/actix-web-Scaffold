/*! 异常处理 */

use actix_web::{
    error,
    http::{header::ContentType, StatusCode},
    HttpResponse,
};
use derive_more::{Display, Error};

#[derive(Debug, Display, Error)]
pub enum CustomerError {
    #[display(fmt = "Business error: {}", msg)]
    BussinessError { msg: String },
    #[display(fmt = "Interal error: {}", msg)]
    SystemError { msg: String },
    #[display(fmt = "Validation error on field: {}", field)]
    ValidationError { field: String },
}
impl error::ResponseError for CustomerError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::html())
            .body(self.to_string())
    }
    fn status_code(&self) -> StatusCode {
        match *self {
            CustomerError::SystemError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            CustomerError::ValidationError { .. } => StatusCode::BAD_REQUEST,
            CustomerError::BussinessError { .. } => StatusCode::OK,
        }
    }
}
