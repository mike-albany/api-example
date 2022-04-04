use actix_web::{HttpRequest, HttpResponse, web::{Json, post}, HttpServer, App};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)] // use the Serde crate to allow this struct to be deserialized into, and serialized into
pub struct DeserialiseTo {
    message: String,
    id: i32
}


async fn deserialise(body: Json<DeserialiseTo>) -> HttpResponse {
    //
    // 'body' is a variable of type Json<DeserialiseTo>
    // supplying arguments of type actix_web::web::Json<T> will attempt to deserialise the  
    // body of the request into type T, and return a 502 error to the client if it fails
    // 
     
    HttpResponse::Ok().body(format!("You provided message '{}' with id {}", body.message, body.id)) // create a text response body from the values within the deserialized struct in the request body
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .route("/", post().to(deserialise)) //define a route accepting POST requests ( other type of request will receive a 404) at '/', requests are passed to 'dererialise'
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}