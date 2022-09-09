use std::env;
extern crate dotenv;

use dotenv::dotenv;

use futures::stream::TryStreamExt;
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId, to_bson},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    Client, Collection,
};

use crate::models::task_model::Task;

pub struct MongoRepo {
    col: Collection<Task>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        dotenv().ok();
        let uri: String = match env::var("MONGOURI") {
            Ok(v) => v.to_string(),
            Err(_) => format!("Error loading env variable"),
        };
        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting database");
        let db = client.database("rustDB");
        let col: Collection<Task> = db.collection("Task");

        MongoRepo { col }
    }

    pub async fn create_task(&self, new_task: Task) -> Result<InsertOneResult, Error> {
        let new_doc = Task {
            id: None,
            title: new_task.title,
            description: new_task.description,
            status: new_task.status,
        };
        let task = self
            .col
            .insert_one(new_doc, None)
            .await
            .ok()
            .expect("Error creating task");

        Ok(task)
    }

    pub async fn get_task(&self, id: &String) -> Result<Task, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let task_detail = self
            .col
            .find_one(filter, None)
            .await
            .ok()
            .expect("Error getting task details");

        Ok(task_detail.unwrap())
    }

    pub async fn update_task(&self, id: &String, new_task: Task) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set": {
                "id": new_task.id,
                "title": new_task.title,
                "description": new_task.description,
                "status": to_bson(&new_task.status).unwrap(),
            }
        };
        let updated_doc = self
            .col
            .update_one(filter, new_doc, None)
            .await
            .ok()
            .expect("Error updating task");

        Ok(updated_doc)
    }

    pub async fn delete_task(&self, id: &String) -> Result<DeleteResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let task_detail = self
            .col
            .delete_one(filter, None)
            .await
            .ok()
            .expect("Error deleting task");

        Ok(task_detail)
    }

    pub async fn get_all_tasks(&self) -> Result<Vec<Task>, Error> {
        let mut cursors = self
            .col
            .find(None, None)
            .await
            .ok()
            .expect("Error getting list of tasks");
        let mut tasks: Vec<Task> = Vec::new();
        while let Some(task) = cursors
            .try_next()
            .await
            .ok()
            .expect("Error mapping through cursor")
        {
            tasks.push(task)
        }
        Ok(tasks)
    }
}
