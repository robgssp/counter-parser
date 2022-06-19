#[macro_use] extern crate simple_error;

use actix_web;
use actix_web::{post, HttpServer, App, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use counter_parser::parse;
use counter_parser::eval;

fn eval_line(line: &str) -> Result<String> {
    let expr = parse::best_parse(line).ok_or_else(
        || simple_error!("No good parses in '{}'", line))?;

    eval::eval(&expr, &Default::default()).map(|v| format!("{}\n", v.to_string()))
}

#[derive(Serialize, Deserialize, Debug)]
struct Request {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", rename_all = "camelCase")]
enum Response {
    Good { val: Option<String> },
    Bad { message: String },
}


type Result<A> = std::result::Result<A, Box<dyn std::error::Error>>;

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .service(evalE)
    })
        .bind(("127.0.0.1", 2369))?
        .run()
        .await
}

fn evalE_body(body: String) -> Result<HttpResponse> {
    let req = serde_json::from_str::<Request>(&body)?;

    let resp = match eval_line(&req.message) {
        Ok(str) =>
            HttpResponse::Ok().body(
                serde_json::to_string(
                    &Response::Good { val: Some(str) })?),
        Err(e) =>
            HttpResponse::BadRequest().body(
                serde_json::to_string(
                    &Response::Bad { message: format!("{}", e) })?),
    };
    Ok(resp)
}


#[post("/eval")]
async fn evalE(body: String) -> HttpResponse {
    evalE_body(body).unwrap_or_else(|e| HttpResponse::InternalServerError().body(format!("{}", e)))
}


