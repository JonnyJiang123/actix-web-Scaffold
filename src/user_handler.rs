//! userhandler 处理User
use actix_web::{get, web, Result};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    age: u8,
}
impl User {
    fn new(id: u32, name: String, age: u8) -> User {
        User { id, name, age }
    }
}
#[get("/{id}")]
pub async fn get_by_id(path: web::Path<u32>) -> Result<String> {
    let id = path.into_inner();
    let user = User::new(id, "Rust".to_string(), 18);
    Ok(serde_json::to_string(&user)?)
}
