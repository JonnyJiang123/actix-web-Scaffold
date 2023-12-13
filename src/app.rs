use crate::user_handler;
use actix_web::{
    guard::{Guard, GuardContext},
    web, App, HttpServer,
};
use std::{any::Any, collections::HashMap, sync::RwLock};
pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // 配置mysql
            .app_data(web::Data::new(config_mysql()))
            // 配置redis
            .app_data(web::Data::new(config_redis()))
            // 公共配置
            .app_data(web::Data::new(common_config()))
            // 接口请求计数
            .app_data(web::Data::new(RequestCounter::new()))
            // 注册服务
            .configure(register_service)

        // 注册接口
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

pub struct MysqlPool(HashMap<String, String>);
pub struct RedisConfig(HashMap<String, String>);
#[derive(Debug,Copy, Clone)]
pub struct CommonConfig(HashMap<String, String>);
pub struct RequestContext {
    map: HashMap<String, Box<dyn Any>>,
}
impl RequestContext {
    pub fn new(map: HashMap<String, Box<dyn Any>>) -> RequestContext {
        RequestContext { map }
    }
    pub fn trace(&self) -> &str {
        self.map
            .get("traceId")
            .unwrap()
            .downcast_ref::<String>()
            .unwrap()
            .as_str()
    }
}
async fn init_request_context() -> RequestContext {
    use uuid::Uuid;
    let mut map: HashMap<String, Box<dyn Any>> = HashMap::new();
    map.insert("traceId".to_string(), Box::new(Uuid::new_v4()));
    RequestContext::new(map)
}
/// 接口请求计数
/**
 *
 * # Examples
 * ```rust
 * let rc = RequestCounter::new();
 * rc.count("/hello");
 * ```
 *
 */
#[derive(Debug)]
struct RequestCounter<'a> {
    counter: RwLock<HashMap<&'a str, u32>>,
}
impl<'a> RequestCounter<'a> {
    pub fn new() -> RequestCounter<'a> {
        RequestCounter {
            counter: RwLock::new(HashMap::new()),
        }
    }
    /// 接口请求计数
    pub fn count(&'a mut self, interface: &'a str) {
        let mut lock = self.counter.write().unwrap();
        lock.entry(interface)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }
    pub fn display(&self) {
        let lock = self.counter.read().unwrap();
        println!("request counter now is \n {:#?}", lock)
    }
}

struct AuthorityGuard {}
impl AuthorityGuard {
    pub fn new() -> AuthorityGuard {
        AuthorityGuard {}
    }
}
impl Guard for AuthorityGuard {
    fn check(&self, ctx: &GuardContext<'_>) -> bool {
        true
    }
}
async fn config_mysql() -> MysqlPool {
    MysqlPool(HashMap::new())
}
async fn config_redis() -> RedisConfig {
    RedisConfig(HashMap::new())
}
async fn common_config() -> CommonConfig {
    let mut config = HashMap::new();
    config.insert("port".to_string(),"8080".to_string());
    CommonConfig(config)
}
fn register_service(cfg: &mut web::ServiceConfig) {
    register_user_service(cfg);
    register_order_service(cfg);
    register_product_service(cfg);
}
fn register_user_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(user_handler::get_by_id)
            .service(user_handler::create_user)
            .service(user_handler::query)
            .service(user_handler::create)
            .service(user_handler::create_by_form)
            .service(user_handler::query_config), // .guard(AuthorityGuard::new())
    );
}
fn register_order_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/order"));
}
fn register_product_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/product"));
}
