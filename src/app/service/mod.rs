/*! 所有service集中进行配置 */
use crate::service::{order, product, user};
use actix_web::web;
/// 注册service
pub fn register_service(cfg: &mut web::ServiceConfig) {
    user::register_user_service(cfg);
    order::register_order_service(cfg);
    product::register_product_service(cfg);
}
