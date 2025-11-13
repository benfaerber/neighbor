//! # Bin Packing Implementation
//!
//! The bin packing problem is an optimization problem where items of different sizes must be
//! packed into bins or containers with fixed capacity, minimizing the number of bins used.
//!
//! For more information, see: https://en.wikipedia.org/wiki/Bin_packing_problem

use crate::model::{Listing, PossibleSpace, Vehicle};
use std::collections::HashMap;

/// Main search function that finds all possible locations for the given vehicles
pub fn search_locations(vehicles: Vec<Vehicle>, listings: &[Listing]) -> Vec<PossibleSpace> {
    let expanded_vehicles = expand_vehicles(vehicles);

    if expanded_vehicles.is_empty() {
        return Vec::new();
    }

    let grouped = group_by_location(listings);
    let mut results = Vec::new();

    for (location_id, location_listings) in grouped {
        if let Some(CheapestCombo { listing_ids, total_price_in_cents }) =
            find_cheapest_combination(&expanded_vehicles, &location_listings) {
            results.push(PossibleSpace {
                location_id,
                listing_ids,
                total_price_in_cents,
            });
        }
    }

    results.sort_by_key(|r| r.total_price_in_cents);
    results
}

pub fn expand_vehicles(vehicles: Vec<Vehicle>) -> Vec<i32> {
    let mut expanded = Vec::new();
    for vehicle in vehicles {
        for _ in 0..vehicle.quantity {
            expanded.push(vehicle.length);
        }
    }
    expanded
}

fn group_by_location(listings: &[Listing]) -> HashMap<String, Vec<Listing>> {
    let mut grouped: HashMap<String, Vec<Listing>> = HashMap::new();
    for listing in listings {
        grouped
            .entry(listing.location_id.clone())
            .or_insert_with(Vec::new)
            .push(listing.clone());
    }
    grouped
}

pub struct CheapestCombo {
    pub listing_ids: Vec<String>,
    pub total_price_in_cents: i32,
} 

impl CheapestCombo {
    fn from_listings(listings: &Vec<Listing>, total_price_in_cents: i32) -> Self {
        let listing_ids = listings.iter().map(|l| l.id.clone()).collect();
        Self { listing_ids, total_price_in_cents }
    }
}

/// Find the cheapest combination of listings that can fit all vehicles
pub fn find_cheapest_combination(
    vehicles: &[i32],
    listings: &[Listing],
) -> Option<CheapestCombo> {
    let n = listings.len();

    // TODO: It doesn't mention max length validation in the readme?
    // if n > 20 {
    //     return None;
    // }

    let mut best: Option<CheapestCombo> = None;

    // Use a powerset to try all the different combos
    // More info: https://www.geeksforgeeks.org/dsa/power-set/
    for mask in 1..(1 << n) {
        let mut selected_listings = Vec::new();
        let mut total_price = 0;

        for i in 0..n {
            if (mask & (1 << i)) != 0 {
                selected_listings.push(listings[i].clone());
                total_price += listings[i].price_in_cents;
            }
        }

        if can_fit_all_vehicles(vehicles, &selected_listings) {
            match &best {
                Some(CheapestCombo { total_price_in_cents: best_price, .. }) if total_price < *best_price => {
                    best = Some(CheapestCombo::from_listings(&selected_listings, total_price));
                }
                None => {
                    best = Some(CheapestCombo::from_listings(&selected_listings, total_price));
                }
                _ => {}
            }
        }
    }

    best
}

/// Check if all vehicles can fit in the given listings
pub fn can_fit_all_vehicles(vehicles: &[i32], listings: &[Listing]) -> bool {
    let mut assignment = vec![None; vehicles.len()];

    fn backtrack_assign(
        vehicles: &[i32],
        listings: &[Listing],
        assignment: &mut Vec<Option<usize>>,
        vehicle_idx: usize,
    ) -> bool {
        if vehicle_idx == vehicles.len() {
            return true;
        }

        let vehicle_length = vehicles[vehicle_idx];

        for (idx, listing) in listings.iter().enumerate() {
            if can_add_vehicle_to_listing(
                vehicles,
                &listing,
                assignment,
                idx,
                vehicle_length,
            ) {
                assignment[vehicle_idx] = Some(idx);
                if backtrack_assign(vehicles, listings, assignment, vehicle_idx + 1) {
                    return true;
                }
                assignment[vehicle_idx] = None;
            }
        }

        false
    }

    backtrack_assign(vehicles, listings, &mut assignment, 0)
}


fn can_add_vehicle_to_listing(
    vehicles: &[i32],
    listing: &Listing,
    assignment: &[Option<usize>],
    listing_idx: usize,
    new_vehicle_length: i32,
) -> bool {

    let mut assigned_vehicles = Vec::new();
    for (i, assigned) in assignment.iter().enumerate() {
        if let Some(idx) = assigned {
            if *idx == listing_idx {
                assigned_vehicles.push(vehicles[i]);
            }
        }
    }

    let (primary, secondary) = get_orientation(listing); 

    try_fit_vehicles_in_dimension(
        &assigned_vehicles,
        new_vehicle_length,
        primary,
        secondary,
    )
}

fn get_orientation(listing: &Listing) -> (i32, i32) {
    if listing.length >= listing.width {
        (listing.length, listing.width)
    } else {
        (listing.width, listing.length)
    }
}

pub fn try_fit_vehicles_in_dimension(
    existing_vehicles: &[i32],
    new_vehicle: i32,
    primary_dim: i32,
    secondary_dim: i32,
) -> bool {
    if Vehicle::WIDTH > secondary_dim {
        return false;
    }

    let num_rows = secondary_dim / Vehicle::WIDTH;
    if num_rows == 0 {
        return false;
    }

    // Greedy bin packing reference: https://github.com/solomon-b/greedypacker 
    let mut all_vehicles: Vec<i32> = existing_vehicles.to_vec();
    all_vehicles.push(new_vehicle);
    all_vehicles.sort_by(|a, b| b.cmp(a));

    let mut rows: Vec<i32> = vec![0; num_rows as usize];
    for &vehicle_length in &all_vehicles {
        let mut placed = false;

        for row in rows.iter_mut() {
            if *row + vehicle_length <= primary_dim {
                *row += vehicle_length;
                placed = true;
                break;
            }
        }

        if !placed {
            return false;
        }
    }

    true
}

