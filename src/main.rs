#[macro_use]
extern crate rocket;

mod error {
    use rocket::http::Status;
    use rocket::request::Request;
    use rocket::response::{self, status, Responder};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {}

    impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
        fn respond_to(self, req: &'r Request<'_>) -> response::Result<'o> {
            let response = match self {
                Error::Unprocessable(ref problems) => {
                    let v =
                        serde_json::to_value(problems).map_err(|_| Status::InternalServerError)?;

                    let response = rocket::Response::build_from(v.respond_to(req)?)
                        .status(Status::UnprocessableEntity)
                        .finalize();

                    Ok(response)
                }
                _ => Status::InternalServerError.respond_to(&req),
            };

            self.capture();

            response
        }
    }
}

mod todos {
    pub struct Todo {
        userId: u32,
        id: u32,
        title: String,
        completed: bool,
    }

    pub fn fetch(id: u32) -> Result<Todo, crate::Error> {
        todo!()
    }
}

use error::Error;

#[get("/todo/<id>")]
fn todo(id: i32) -> Result<&'static str, Error> {
    Ok("hello world!")
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![todo])
}
