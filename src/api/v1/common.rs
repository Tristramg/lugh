use errors::*;
use iron::prelude::*;
use models::*;
use params::{Params, Value};

pub fn current_user(request: &Request) -> Result<User, StringError> {
    use models::Session;

    let current_session = request.extensions.get::<Session>().unwrap();

    current_session.user()
}

pub fn get_param(request: &mut Request, name: &str) -> Result<String, BadRequestError> {
    let parameters = request.get_ref::<Params>().unwrap();

    match parameters.find(&[name]) {
        Some(&Value::String(ref value)) => Ok(value.clone()),
        _ => Err(BadRequestError(format!("You must provide a {}", name))),
    }
}
