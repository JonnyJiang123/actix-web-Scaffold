use actix_web_Scaffold::app;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    app::run().await
}
