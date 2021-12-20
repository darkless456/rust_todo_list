use rocket::Responder;

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
pub struct Correct(String);

impl Correct {
    pub fn new(content: String) -> Correct {
      Correct(content)
    }
}