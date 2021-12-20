use std::env;
use backend::db::{ 
  create_task, 
  establish_connection, 
  query_tasks,
  update_task_by_id,
  delete_task_by_id,
};

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("missing arguments");
    return;
  }

  let subcommand = &args[1];
  match subcommand.as_ref() {
    "new" => {
      new_task(&args[2..]);
    },
    "show" => {
      show_tasks(&args[2..]);
    },
    "update" => {
      update_task(&args[2..]);
    },
    "delete" => {
      delete_task(&args[2..]);
    },
    command => {
      println!("{} is unexpected command!", command);
    }
  }
}

fn new_task(args: &[String]) {
  if args.len() < 1 {
    println!("new: missing <title>");
    return;
  }
  
  let connection = establish_connection();

  let title;
  let body: Option<&str>;
  if args.len() >= 2 {
    title = &args[0];
    body = Some(&args[1]);
  } else {
    title = &args[0];
    body = None;
  }

  create_task(&connection, title, body);
}

fn show_tasks(args: &[String]) {
  if args.len() > 0 {
    println!("show: unexpected argument");
    return;
  }

  let connection = establish_connection();
  println!("All Tasks:");
  for task in query_tasks(&connection) {
    println!("------\r\nid: {},\r\ntitle: {},\r\nbody: {}", task.id, task.title, task.body);
  }
}

fn update_task(args: &[String]) {
  if args.len() < 1 {
    println!("update: missing id argument");
    return
  }

  let id = args[0].parse::<i32>().expect("Invalid ID");
  let connection = establish_connection();
  update_task_by_id(&connection, id);
}


fn delete_task(args: &[String]) {
  if args.len() < 1 {
    println!("delete: missing id argument");
    return
  }

  let id = args[0].parse::<i32>().expect("Invalid ID");
  let connection = establish_connection();
  delete_task_by_id(&connection, id);
}