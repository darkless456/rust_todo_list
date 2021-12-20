use super::schema::tasks;
use diesel::{ Insertable, Queryable };
use serde_json;


#[derive(Insertable)]
#[table_name = "tasks"]
pub struct NewTask<'a> {
  pub title: &'a str,
  pub body: &'a str,
}

impl<'a> NewTask<'a> {
  pub fn new(title: &'a str, body: Option<&'a str>) -> NewTask<'a> {
    let new_task;
    if let Some(body) = body {
      new_task = NewTask { title, body };
    } else {
      new_task = NewTask { title, body: "no text" };
    }
    return new_task;
  }
}

#[derive(Queryable, Serialize)]
pub struct Task {
  pub id: i32,
  pub title: String,
  pub body: String,
}

impl Task {
  pub fn to_json_string(&self) -> String {
    serde_json::to_string(&self).unwrap()
  }
}
