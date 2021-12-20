#[macro_use]
extern crate rocket;

use serde_json;

use rocket::{
    fs::FileServer,
    response::status,
    serde::json::Json,
    Build, Request, Rocket,
};
use std::path::PathBuf;
use backend::{
    api::{models::Task, response_types::Correct},
    db::{establish_connection, query_tasks},
};

#[catch(404)]
fn not_found(req: &Request) -> String {
    format!("Sorry, '{}' is not a valid path.", req.uri())
}

#[get("/tasks")]
fn tasks_get() -> Correct {
    let connection = establish_connection();
    let all_tasks = query_tasks(&connection);

    // let mut response = vec![];
    // for task in all_tasks {
    //     response.push(task);
    // }

    Correct::new(serde_json::to_string(&all_tasks).unwrap())
    // Json(all_tasks)
}

#[post("/task/create", data = "<task>")]
fn task_create(task: Json<Task>) -> status::Accepted<String> {
    status::Accepted(Some(format!(
        "Task Created, title: {}, content: {}",
        task.title, task.body
    )))
}

#[put("/task/<id>", data = "<task>")]
fn task_update(id: u32, task: String) -> String {
    let id = id.to_string();
    format!("{} Task Updated, Content: {}", id, task)
}

#[delete("/task/<id>")]
fn task_delete(id: u32) -> String {
    let id = id.to_string();
    format!("{} Task Deleted!", id)
}

#[get("/")]
fn setting_index() -> &'static str {
    "Setting Page"
}

#[get("/<path..>")]
fn setting_config(path: PathBuf) -> String {
    format!("uncertain path params, {:#?}", path.into_os_string())
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![tasks_get, task_create, task_update, task_delete],
        )
        .mount("/setting", routes![setting_index, setting_config,])
        .mount("/public", FileServer::from("static/"))
        .register("/", catchers![not_found])
}
