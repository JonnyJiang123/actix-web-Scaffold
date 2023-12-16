/*! 请求处理 */
use std::any::Any;
use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    web, Error, HttpMessage
};
use futures_util::future::{LocalBoxFuture, ok};
use tracing::{span,Level,event};
use std::future::{Future, ready, Ready};
use std::str::FromStr;
use actix_web::body::{BoxBody, MessageBody};

use serde_json;

pub struct RequestHandler;
impl<S, B> Transform<S, ServiceRequest> for RequestHandler
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = RequestHandlerMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(RequestHandlerMiddleware { service }))
    }
}

pub struct RequestHandlerMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for RequestHandlerMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let span = span!(Level::TRACE,"test");
        let _enter = span.enter();
        let path = req.path();
        event!(Level::INFO,"Hi from start. You requested: {}", path);
        let query_params = req.query_string();
        event!(Level::INFO,"Query parameters: {:?}", query_params);

        let path_params = req.match_info();
        event!(Level::INFO,"Path parameters: {:?}", path_params);

        let fut  = self.service.call(req);

        Box::pin(async move {
            let res= fut.await?;
            let (req, res) = res.into_parts();
            let (res, body) = res.into_parts();
            let temp_res = ServiceResponse::new(req, res.map_into_boxed_body());
            // re-construct original service response
            let (req, res) = temp_res.into_parts();

            Ok(ServiceResponse::new(req, res.set_body(body)))
        })
    }
}
