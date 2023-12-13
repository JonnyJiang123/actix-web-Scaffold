use crate::app::{AppState, CommonConfig};
use actix_web::{get, post, web, Result};
use serde::{Deserialize, Serialize};

pub fn register_user_service(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .service(get_by_id)
            .service(create_user)
            .service(query)
            .service(create)
            .service(create_by_form)
            .service(query_config), // .guard(AuthorityGuard::new())
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

#[post("/create")]
pub async fn create(json: web::Json<User>) -> Result<String> {
    let user = json.into_inner();
    Ok(serde_json::to_string(&user)?)
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
