mod api;
mod models;
mod repository;

use actix_web::{web::Data, App, HttpServer};
use api::task_api::{create_task, delete_task, get_all_tasks, get_task, update_task};
use repository::mongodb_repo::MongoRepo;

pub struct Server {
    port: u16,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Server { port }
    }

    pub async fn run(&self) -> std::io::Result<()> {
        let db = MongoRepo::init().await;
        let db_data = Data::new(db);
        HttpServer::new(move || {
            App::new()
                .app_data(db_data.clone())
                .service(create_task)
                .service(get_task)
                .service(update_task)
                .service(delete_task)
                .service(get_all_tasks)
        })
        .bind(("127.0.0.1", self.port))?
        .run()
        .await
    }
}
