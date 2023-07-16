pub mod error;
pub mod macros;
pub mod operations;
pub mod utils;

use actix_web::{
    web::{post, resource, Json, ServiceConfig},
    HttpResponse,
};
use serde::{Deserialize, Serialize};

use operations::A1Z26CipherDecode::A1Z26CipherDecoder;
use operations::Operation;

#[derive(Serialize, Deserialize)]
pub struct Request {
    method: String,
    input: String,
}

pub async fn root(req: Json<Request>) -> HttpResponse {

    if req.method == "A1Z26CipherDecoder" {
        let answer = A1Z26CipherDecoder(
            Operation {
                lang: "en".to_string(),
                params: vec![" ".to_string()],
                input: req.input.clone()
            }
        );
        return match answer {
            Ok(a) => HttpResponse::Ok().json(a),
            Err(e) => HttpResponse::Ok().json(e.to_string())
        }
    };

    HttpResponse::Ok().json("Hey Hey Hey")
}

pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(resource("/api").route(post().to(root)));
}
