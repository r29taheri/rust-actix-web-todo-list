use crate::{
    models::task_model::{Status, Task},
    repository::mongodb_repo::MongoRepo,
};
use actix_web::{
    delete, get, post, put,
    web::{Data, Json, Path},
    HttpResponse,
};
use mongodb::bson::oid::ObjectId;

#[post("/task")]
pub async fn create_task(db: Data<MongoRepo>, new_task: Json<Task>) -> HttpResponse {
    let data = Task {
        id: None,
        title: new_task.title.to_owned(),
        description: new_task.description.to_owned(),
        status: Status::Todo,
    };
    let task_detail = db.create_task(data).await;
    match task_detail {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/task/{id}")]
pub async fn get_task(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let task_detail = db.get_task(&id).await;
    match task_detail {
        Ok(task) => HttpResponse::Ok().json(task),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("task/{id}")]
pub async fn update_task(
    db: Data<MongoRepo>,
    path: Path<String>,
    new_task: Json<Task>,
) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("Invalid ID");
    }

    let data = Task {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        title: new_task.title.to_owned(),
        description: new_task.description.to_owned(),
        status: new_task.status.to_owned(),
    };
    let update_result = db.update_task(&id, data).await;
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_task_info = db.get_task(&id).await;
                return match updated_task_info {
                    Ok(task) => HttpResponse::Ok().json(task),
                    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
                };
            } else {
                return HttpResponse::NotFound().body("No task found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[delete("/task/{id}")]
pub async fn delete_task(db: Data<MongoRepo>, path: Path<String>) -> HttpResponse {
    let id = path.into_inner();
    if id.is_empty() {
        return HttpResponse::BadRequest().body("invalid ID");
    };

    let result = db.delete_task(&id).await;
    match result {
        Ok(res) => {
            if res.deleted_count == 1 {
                return HttpResponse::Ok().json("Task is deleted");
            } else {
                return HttpResponse::NotFound().json("No task found with specified ID");
            }
        }
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/tasks")]
pub async fn get_all_tasks(db: Data<MongoRepo>) -> HttpResponse {
    let tasks = db.get_all_tasks().await;
    match tasks {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}
