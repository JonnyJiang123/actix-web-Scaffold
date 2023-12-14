/*! product service */
use actix_web::web;
pub fn register_product_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/product"));
}
