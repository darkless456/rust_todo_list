use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub mod models;
mod schema;

pub fn establish_connection() -> MysqlConnection {
  dotenv().ok();
  let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  MysqlConnection::establish(&url).unwrap_or_else(|_| {
    panic!("connect to {} fail", &url);
  })
}

pub fn create_task(connection: &MysqlConnection, title: &str, body: Option<&str>) {
  use models::NewTask;
  let new_task = NewTask::new(title, body);

  diesel::insert_into(schema::tasks::table)
    .values(&new_task)
    .execute(connection)
    .expect("error while inserting a new task");
}

pub fn query_tasks(connection: &MysqlConnection) -> Vec<models::Task> {
  schema::tasks::table
    .load::<models::Task>(connection)
    .expect("error while loading a task")
}

use schema::tasks::dsl;

pub fn update_task_by_id(connection: &MysqlConnection, id: i32) {
  let effect_rows = diesel::update(dsl::tasks.find(id))
    .set(dsl::body.eq("update content"))
    .execute(connection)
    .expect("error while updating task");
  println!("update {} tasks", effect_rows)
}

pub fn delete_task_by_id(connection: &MysqlConnection, id: i32) {
  let effect_rows = diesel::delete(dsl::tasks.find(id))
    .execute(connection)
    .expect("error while deleting task");
  println!("delete {} tasks", effect_rows);
}