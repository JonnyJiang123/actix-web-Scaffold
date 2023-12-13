//! userhandler 处理User
use actix_web::{get, post, web, Result};
use serde::{Deserialize, Serialize};
use crate::app::CommonConfig;

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
/// 提前路径参数
#[get("/query/{id}")]
pub async fn get_by_id(path: web::Path<u32>,common_config: web::Data<CommonConfig>) -> Result<String> {
    let id = path.into_inner();
    let user = User::new(id, "Rust".to_string(), 18);
    println!("common config: {:?}",&common_config.into_inner());
    Ok(serde_json::to_string(&user)?)
}
/// 将路径参数封装到struct
#[post("/create/{id}/{name}/{age}")]
pub async fn create_user(path: web::Path<User>) -> Result<String> {
    let user = path.into_inner();
    println!("created user is {}", serde_json::to_string(&user)?);
    Ok("OK".to_string())
}
/// 通过提取请求参数
#[get("/query")]
pub async fn query(user_query: web::Query<UserQuery>) -> Result<String> {
    let query = user_query.into_inner();
    println!("user  query is {:?}", query);
    Ok("OK".to_string())
}
///提前请求体json数据
#[post("/create")]
pub async fn create(json:web::Json<User>)->Result<String>{
    let user = json.into_inner();
    Ok(serde_json::to_string(&user)?)
}

/// 通过表单提交
#[post("/create_by_form")]
pub async fn create_by_form(form:web::Form<User>)->Result<String>{
    let data = form.into_inner();
    Ok(serde_json::to_string(&data)?)
}

#[get("config")]
pub async fn query_config(config:web::Data<CommonConfig>) -> Result<String>{
    let config = config.get_ref();
    println!("config is {:?}",config);
    Ok(String::from("OK"))
}