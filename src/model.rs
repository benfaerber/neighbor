use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use validator::{Validate, ValidationError};

lazy_static! {
    static ref ALL_LISTINGS: AllListings = AllListings::load() 
        .expect("Missing listings.json config file!");
}

pub struct AllListings(Vec<Listing>);

impl AllListings {
    pub fn load() -> anyhow::Result<Self> {
        let data = fs::read_to_string("listings.json")?;
        let listings: Vec<Listing> = serde_json::from_str(&data)?;
        Ok(Self(listings))
    }

    /// Get the singleton
    pub fn get() -> &'static AllListings {
        &ALL_LISTINGS
    }

    pub fn inner(&'static self) -> &'static [Listing] {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Listing {
    pub id: String,
    pub location_id: String,
    /// Multiple of 10
    pub length: i32,
    /// Multiple of 10
    pub width: i32,
    pub price_in_cents: i32,
}

fn validate_length(length: i32) -> Result<(), ValidationError> {
    if length <= 0 {
        return Err(ValidationError::new("length_must_be_positive"));
    }
    Ok(())
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct Vehicle {
    #[validate(custom(function = "validate_length"))]
    pub length: i32,
    #[validate(range(min = 1))]
    pub quantity: i32,
}

fn validate_total_quantity(vehicles: &[Vehicle]) -> Result<(), ValidationError> {
    let total: i32 = vehicles.iter().map(|v| v.quantity).sum();
    if total > 5 {
        return Err(ValidationError::new("total_quantity_exceeds_5"));
    }
    Ok(())
}

/// Note: The endpoint accepts an array.
/// I flatten it using transparent
#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(transparent)]
pub struct SearchRequest {
    #[validate(length(min = 1), nested, custom(function = "validate_total_quantity"))]
    pub vehicles: Vec<Vehicle>,
}

impl From<SearchRequest> for Vec<Vehicle> {
    fn from(request: SearchRequest) -> Self {
        request.vehicles
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PossibleSpace {
    pub location_id: String,
    pub listing_ids: Vec<String>,
    pub total_price_in_cents: i32,
}
