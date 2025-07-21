// trying to code a simple web server 
// in actix-web 
use std::io::Result;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};

// define the Request struct usign serialize 
#[derive(Serialize, Deserialize)]
struct RequestData {
    id: u8,
    username: String,
    password: String
}

// we're also defining the Response we're going to send
#[derive(serde::Serialize)]
struct ResponseData {
    text: String,
    received_data: RequestData
}

// write the get & post handlers 
async fn get() -> impl Responder {
    HttpResponse::Ok().body("hello from GET handler")
}

// then we want our handler to have some request data so our post fn will take the Struct RequestData as a parameter
async fn post(data: web::Json<RequestData>) -> impl Responder {
    let received_data = data.into_inner();
    let response_data = ResponseData {
        text: "Below is the Response in JSON formatting".to_string(),
        received_data: received_data
    };
    HttpResponse::Ok().json(response_data)
}

// write the main fn logic 
#[actix_web::main]  // to make the main fn asycn 

async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
        // register the above handlers, & we'll do that using .route built-in function
        .route("/", web::get().to(get))
        .route("/post", web::post().to(post))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}