/*！user_service  */
use crate::common::dto::{RestRequest, RestResponse};
use crate::common::err::CustomerError;
use crate::config::common::CommonConfig;
use actix_web::{get, post, web, Responder, Result};
use log;
use serde::{Deserialize, Serialize};

pub fn register_user_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(get_by_id)
            .service(create_user)
            .service(query)
            .service(create)
            .service(create_by_form)
            .service(query_config)
            .service(error_test), // .guard(AuthorityGuard::new())
    );
}

#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    age: u8,
}

#[derive(Debug, Deserialize)]
struct UserQuery {
    pub name: String,
}

impl User {
    fn new(id: u32, name: String, age: u8) -> User {
        User { id, name, age }
    }
}

#[get("/query/{id}")]
pub async fn get_by_id(
    path: web::Path<u32>,
    common_config: web::Data<CommonConfig>,
) -> Result<String> {
    let id = path.into_inner();
    let user = User::new(id, "Rust".to_string(), 18);
    println!("common config: {:?}", &common_config);
    Ok(serde_json::to_string(&user)?)
}

#[post("/create/{id}/{name}/{age}")]
pub async fn create_user(path: web::Path<User>) -> Result<String> {
    let user = path.into_inner();
    println!("created user is {}", serde_json::to_string(&user)?);
    Ok("OK".to_string())
}

#[get("/query")]
pub async fn query(user_query: web::Query<UserQuery>) -> Result<String> {
    let query = user_query.into_inner();
    println!("user query is {:?}", query);
    Ok("OK".to_string())
}
/// 使用标准的RestRequest、RestResponse
#[post("/create")]
pub async fn create(json: web::Json<RestRequest<User>>) -> Result<impl Responder> {
    let user = json.into_inner();
    let response = RestResponse::new(user);
    Ok(web::Json(response))
}

#[post("/create_by_form")]
pub async fn create_by_form(form: web::Form<User>) -> Result<String> {
    let data = form.into_inner();
    Ok(serde_json::to_string(&data)?)
}

#[get("/config")]
pub async fn query_config(config: web::Data<CommonConfig>) -> Result<String> {
    let config = config.get_ref();
    println!("config is {:?}", config);
    Ok(String::from("OK"))
}
#[get("/error_test")]
pub async fn error_test() -> Result<impl Responder, CustomerError> {
    let err = false;
    if err {
        return Ok(web::Json(RestResponse::new("ok")));
    }
    let err = CustomerError::ValidationError {
        field: "name".to_string(),
    };
    log::error!("{}", err);
    Err(err)
}
