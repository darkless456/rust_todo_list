#[derive(Deserialize, Serialize,)]
pub struct Task<'a> {
  pub title: &'a str,
  pub body: &'a str,
}

// impl<'a> Task<'a> {
//   pub fn to_json_string(&self) -> &Task {
//     println!("task to json: {:?}", self);
//     return self
//   }
// }