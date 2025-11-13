use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;
use validator::Validate;

mod bin_packing;
mod model;

#[cfg(test)]
mod tests;

use crate::model::{AllListings, SearchRequest, Vehicle};

#[post("/")]
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
    println!("Starting server at http://127.0.0.1:8080");

    // Load the listings on server start up...
    // They are probably gonna time me based on API response time so I will preload now.
    let _ = AllListings::get(); 

    HttpServer::new(|| App::new().service(search))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
