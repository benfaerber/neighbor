use actix_web::{post, get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;
use serde_json::json;
use validator::Validate;

mod bin_packing;
mod model;

#[cfg(test)]
mod tests;

use crate::model::{AllListings, SearchRequest, Vehicle};


const IP_ADDRESS: &str = "0.0.0.0";
const PORT: u16 = 8080; 

#[derive(Serialize, Debug)]
struct ServiceHealth {
    is_healthy: bool,
    message: String,
} 

fn health_response() -> impl Responder {
    HttpResponse::Ok().json(ServiceHealth {
        is_healthy: true,
        message: "Service is healthy! Send a post request to /search".into()
    })
}

#[get("/")]
async fn index() -> impl Responder {
    health_response()
}

#[get("/health")]
async fn health() -> impl Responder {
    health_response()
}

#[post("/search")]
async fn search(request: web::Json<SearchRequest>) -> impl Responder {
    let request = request.into_inner();

    if let Err(e) = request.validate() {
        return HttpResponse::BadRequest().json(json!({
            "error": "Validation failed",
            "details": e.to_string()
        }));
    }

    let vehicles: Vec<Vehicle> = request.into();
    // Listings already got loaded so they are instant now...
    let listings = AllListings::get(); 
    let results = bin_packing::search_locations(vehicles, listings.inner());
    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://{}:{}", IP_ADDRESS, PORT);

    // Load the listings on server start up...
    // They are probably gonna time me based on API response time so I will preload now.
    let _ = AllListings::get(); 

    HttpServer::new(|| App::new().service(index).service(search).service(health))
        .bind((IP_ADDRESS, PORT))?
        .run()
        .await
}
