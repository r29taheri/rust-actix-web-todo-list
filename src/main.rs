#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app = actix_task::Server::new(8080);
    app.run().await
}
