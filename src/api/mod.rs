pub mod error;
pub mod macros;
pub mod operations;
pub mod utils;

use actix_web::{
    web::{post, resource, Json, ServiceConfig},
    HttpResponse,
};
use error::Error;
use operations::Operation;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    lang: String,
    method: String,
    input: String,
}

impl Request {
    pub fn parse(&self) -> Result<Operation, Error> {
        let mut method = self.method.splitn(2, '(');

        let name = match method.next() {
            Some(name) => name.to_string(),
            None => return Err(Error::MissingParameterError),
        };
        let lang = self.lang.to_owned();
        let input = self.input.to_owned();
        let mut params: Vec<String> = match method.next() {
            Some(params) => params
                .to_string()
                .split(',')
                .map(|mut param| {
                    if param.ends_with(")") {
                        param = &param[..param.len() - 1];
                    }
                    param
                        .trim_matches(|c| c == '\'' || c == '"')
                        .to_string()
                })
                .collect(),
            None => return Err(Error::MissingParameterError),
        };
        let last = match params.last_mut() {
            Some(last) => last,
            None => return Err(Error::MissingParameterError),
        };
        *last = last.trim_end_matches(')').to_string();

        Ok(Operation {
            name,
            params,
            input,
            lang,
        })
    }
}

pub async fn root(req: Json<Request>) -> HttpResponse {
    HttpResponse::Ok().json("Meow")
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(resource("/api").route(post().to(root)));
}
