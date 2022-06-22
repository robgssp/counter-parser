#[macro_use] extern crate simple_error;
extern crate derive_more;

use actix_web::http::header::ContentType;
use actix_web::{self, web, post, HttpServer, App, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use counter_parser::parse;
use counter_parser::eval;
use counter_parser::types::{Request, Response};
use std::result;
use derive_more::Display;

type Result<A, E = Box<dyn std::error::Error>> = result::Result<A, E>;

#[derive(Debug, Display)]
enum UserError {
    #[display(fmt = "no parses found")]
    NoParse,
    #[display(fmt = "evaluation failed")]
    BadEval,
    #[display(fmt = "internal server error")]
    Internal
}

// impl actix_web::error::ResponseError for UserError {
//     fn error_response(&self) -> HttpResposne {
//         HttpResponse::build(self.status_code())
//             .

#[actix_web::main]
async fn main() -> std::result::Result<(), std::io::Error> {
    println!("Starting counter-parser server...");
    HttpServer::new(|| {
        App::new()
            .service(eval_svc)
    })
        .bind(("127.0.0.1", 2369))?
        .run()
        .await
}


#[post("/eval")]
async fn eval_svc(body: String) -> Result<HttpResponse> {
    let req: Request = serde_json::from_str(&body)?;

    let expr = parse::best_parse(&req.message).ok_or_else(
        || simple_error!("No parses found"))?;

    let val = eval::eval(&expr, &Default::default())?;

    Ok(
        HttpResponse::Ok().body(
            serde_json::to_string(
                &Response::Good { val: Some(format!("{}", val)) })?)
            //.set(ContentType::JSON)
    )
}


