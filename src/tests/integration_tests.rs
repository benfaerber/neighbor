//! This is using all the info I was given on the README
//! The solution should pass for each README example 

use crate::bin_packing;
use crate::model::{AllListings, Vehicle};

use std::collections::HashSet;

#[test]
fn test_readme_example() {
    // Example from README: single vehicle of length 10
    let vehicles = vec![Vehicle {
        length: 10,
        quantity: 1,
    }];

    let listings = AllListings::get();
    let results = bin_packing::search_locations(vehicles, listings.inner());

    assert!(!results.is_empty(), "Should return at least one result");
    assert!(
        results.len() >= 364,
        "Expected at least 364 results for single 10ft vehicle, got {}",
        results.len()
    );

    for i in 0..results.len() - 1 {
        assert!(
            results[i].total_price_in_cents <= results[i + 1].total_price_in_cents,
            "Results should be sorted by price ascending"
        );
    }

    // Verify first result (cheapest)
    let first = &results[0];
    assert_eq!(
        first.location_id, "42b8f068-2d13-4ed1-8eec-c98f1eef0850",
        "First location_id should match README example"
    );
    assert_eq!(
        first.listing_ids,
        vec!["b9bbe25f-5679-4917-bd7b-1e19c464f3a8"],
        "First listing_ids should match README example"
    );
    assert_eq!(
        first.total_price_in_cents, 1005,
        "First price should match README example"
    );

    // Verify second result
    let second = &results[1];
    assert_eq!(
        second.location_id, "507628b8-163e-4e22-a6a3-6a16f8188928",
        "Second location_id should match README example"
    );
    assert_eq!(
        second.listing_ids,
        vec!["e7d59481-b804-4565-b49b-d5beb7aec350"],
        "Second listing_ids should match README example"
    );
    assert_eq!(
        second.total_price_in_cents, 1088,
        "Second price should match README example"
    );

    // The middle results are not included so we can skip those...

    // Verify last result (most expensive)
    let last = &results[results.len() - 1];
    assert_eq!(
        last.location_id, "22ad1ab7-d49b-49d6-8c30-531599934639",
        "Last location_id should match README example"
    );
    assert_eq!(
        last.listing_ids,
        vec!["20cf6f5e-eb47-4104-b1f9-62527760a4c0"],
        "Last listing_ids should match README example"
    );
    assert_eq!(
        last.total_price_in_cents, 99303,
        "Last price should match README example"
    );
}

#[test]
fn test_readme_complex_example() {
    // Complex example from README: multiple vehicles
    let vehicles = vec![
        Vehicle {
            length: 10,
            quantity: 1,
        },
        Vehicle {
            length: 20,
            quantity: 2,
        },
        Vehicle {
            length: 25,
            quantity: 1,
        },
    ];

    let listings = AllListings::get(); 
    let results = bin_packing::search_locations(vehicles, listings.inner());

    // Should return some results
    assert!(!results.is_empty(), "Should find locations that fit all vehicles");

    // Verify results are sorted by price
    for i in 0..results.len() - 1 {
        assert!(
            results[i].total_price_in_cents <= results[i + 1].total_price_in_cents,
            "Results should be sorted by price ascending"
        );
    }

    // Verify each result includes only one result per location_id
    let mut seen_locations = HashSet::new();
    for result in &results {
        assert!(
            !seen_locations.contains(&result.location_id),
            "Should have only one result per location_id"
        );
        seen_locations.insert(result.location_id.clone());
    }

    for result in &results {
        assert!(
            !result.listing_ids.is_empty(),
            "Each result should have at least one listing"
        );
    }
}
