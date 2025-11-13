use lazy_static::lazy_static;
use serde::{Serialize, Deserialize};
use std::fs;

lazy_static! {
    static ref ALL_LISTINGS: Listings = Listings::load()
        .expect("Missing listings.json config file!");
}

pub struct Listings {
    listings: Vec<Listing>,
}

impl Listings {
    pub fn load() -> anyhow::Result<Self> {
        let data = fs::read_to_string("listings.json")?;
        let listings: Vec<Listing> = serde_json::from_str(&data)?;
        Ok(Listings { listings })
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Listing {
    id: String,
    location_id: String,
    /// Multiple of 10
    length: i32,
    /// Multiple of 10
    width: i32,
    price_in_cents: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Vehicle {
    length: i32,
    quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PossibleSpace {
    location_id: String,
    listings_id: Vec<String>,
    total_price_in_cents: i32,
}
