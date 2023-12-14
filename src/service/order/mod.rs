/*! user service */
use actix_web::web;
pub fn register_order_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/order"));
}
