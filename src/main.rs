use actix_web_Scaffold::{app, common::dto::RestRequest};
use serde::Deserialize;
use serde_json;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
