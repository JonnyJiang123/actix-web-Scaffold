/*! 应用相关配置 */
mod guard;
mod middleware;
mod service;
mod state;
use crate::config::{common, db};
use actix_web::{web, App, HttpServer};
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            // 配置mysql
            .app_data(web::Data::new(db::MysqlPool::config_mysql()))
            // 配置redis
            .app_data(web::Data::new(db::RedisConfig::config_redis()))
            // 公共配置
            .app_data(web::Data::new(common::CommonConfig::common_config()))
            // 注册服务
            .configure(service::register_service)

        // 注册接口
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
