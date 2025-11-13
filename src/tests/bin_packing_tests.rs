//! Test all the functions in bin_packing
//! For the complex README examples see integration_tests

use crate::model::{Vehicle, Listing};
use crate::bin_packing::{self, CheapestCombo}; 

#[test]
fn test_expand_vehicles() {
    let vehicles = vec![
        Vehicle {
            length: 10,
            quantity: 2,
        },
        Vehicle {
            length: 20,
            quantity: 1,
        },
    ];
    let expanded = bin_packing::expand_vehicles(vehicles);
    assert_eq!(expanded, vec![10, 10, 20]);
}

#[test]
fn test_single_vehicle_fits_in_single_listing() {
    let vehicles = vec![10];
    let listings = vec![Listing {
        id: "1".to_string(),
        location_id: "loc1".to_string(),
        length: 20,
        width: 10,
        price_in_cents: 100,
    }];

    assert!(bin_packing::can_fit_all_vehicles(&vehicles, &listings));
}

#[test]
fn test_single_vehicle_does_not_fit() {
    let vehicles = vec![30];
    let listings = vec![Listing {
        id: "1".to_string(),
        location_id: "loc1".to_string(),
        length: 20,
        width: 10,
        price_in_cents: 100,
    }];

    assert!(!bin_packing::can_fit_all_vehicles(&vehicles, &listings));
}

#[test]
fn test_multiple_vehicles_same_listing() {
    let vehicles = vec![10, 10];
    let listings = vec![Listing {
        id: "1".to_string(),
        location_id: "loc1".to_string(),
        length: 20,
        width: 20,
        price_in_cents: 100,
    }];

    assert!(bin_packing::can_fit_all_vehicles(&vehicles, &listings));
}

#[test]
fn test_vehicles_across_multiple_listings() {
    let vehicles = vec![10, 10, 10];
    let listings = vec![
        Listing {
            id: "1".to_string(),
            location_id: "loc1".to_string(),
            length: 10,
            width: 10,
            price_in_cents: 100,
        },
        Listing {
            id: "2".to_string(),
            location_id: "loc1".to_string(),
            length: 20,
            width: 10,
            price_in_cents: 200,
        },
    ];

    assert!(bin_packing::can_fit_all_vehicles(&vehicles, &listings));
}

#[test]
fn test_orientation_uses_longer_dimension() {
    let vehicles = vec![20];
    let listings = vec![Listing {
        id: "1".to_string(),
        location_id: "loc1".to_string(),
        length: 10,
        width: 20,
        price_in_cents: 100,
    }];

    // This should fit because width (20) >= length (10), so width becomes primary dimension
    assert!(bin_packing::can_fit_all_vehicles(&vehicles, &listings));
}

#[test]
fn test_find_cheapest_combination() {
    let vehicles = vec![10];
    let listings = vec![
        Listing {
            id: "1".to_string(),
            location_id: "loc1".to_string(),
            length: 20,
            width: 10,
            price_in_cents: 200,
        },
        Listing {
            id: "2".to_string(),
            location_id: "loc1".to_string(),
            length: 15,
            width: 10,
            price_in_cents: 100,
        },
    ];

    let result = bin_packing::find_cheapest_combination(&vehicles, &listings);
    assert!(result.is_some());
    let CheapestCombo {listing_ids, total_price_in_cents} = result.unwrap();
    assert_eq!(listing_ids, vec!["2"]);
    assert_eq!(total_price_in_cents, 100);
}

#[test]
fn test_cheapest_combination_multiple_listings_needed() {
    let vehicles = vec![30, 30];
    let listings = vec![
        Listing {
            id: "1".to_string(),
            location_id: "loc1".to_string(),
            length: 30,
            width: 10,
            price_in_cents: 100,
        },
        Listing {
            id: "2".to_string(),
            location_id: "loc1".to_string(),
            length: 30,
            width: 10,
            price_in_cents: 150,
        },
        Listing {
            id: "3".to_string(),
            location_id: "loc1".to_string(),
            length: 60,
            width: 20,
            price_in_cents: 500,
        },
    ];

    let result = bin_packing::find_cheapest_combination(&vehicles, &listings);
    assert!(result.is_some());
    let CheapestCombo {listing_ids, total_price_in_cents} = result.unwrap();
    assert_eq!(total_price_in_cents, 250);
    assert!(listing_ids.contains(&"1".to_string()));
    assert!(listing_ids.contains(&"2".to_string()));
}

#[test]
fn test_search_locations_groups_by_location() {
    let vehicles = vec![Vehicle {
        length: 10,
        quantity: 1,
    }];
    let listings = vec![
        Listing {
            id: "1".to_string(),
            location_id: "loc1".to_string(),
            length: 20,
            width: 10,
            price_in_cents: 100,
        },
        Listing {
            id: "2".to_string(),
            location_id: "loc2".to_string(),
            length: 20,
            width: 10,
            price_in_cents: 150,
        },
        Listing {
            id: "3".to_string(),
            location_id: "loc1".to_string(),
            length: 20,
            width: 10,
            price_in_cents: 200,
        },
    ];

    let results = bin_packing::search_locations(vehicles, &listings);

    assert_eq!(results.len(), 2);
    assert!(results[0].total_price_in_cents <= results[1].total_price_in_cents);

    let loc1_result = results.iter().find(|r| r.location_id == "loc1").unwrap();
    assert_eq!(loc1_result.listing_ids, vec!["1"]);
    assert_eq!(loc1_result.total_price_in_cents, 100);
}

#[test]
fn test_try_fit_vehicles_in_dimension() {
    assert!(bin_packing::try_fit_vehicles_in_dimension(&[], 20, 30, 10, 10));
    assert!(!bin_packing::try_fit_vehicles_in_dimension(&[], 40, 30, 10, 10));
    assert!(bin_packing::try_fit_vehicles_in_dimension(&[15], 15, 20, 20, 10));
    assert!(!bin_packing::try_fit_vehicles_in_dimension(&[10, 10], 10, 15, 20, 10));
}
