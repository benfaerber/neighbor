//! Ensure validation is correct.

use crate::model::{SearchRequest, Vehicle};
use validator::Validate;

#[test]
fn test_valid_vehicle() {
    let vehicle = Vehicle {
        length: 10,
        quantity: 1,
    };
    assert!(vehicle.validate().is_ok());
}

#[test]
fn test_valid_vehicle_multiple_of_10() {
    let vehicle = Vehicle {
        length: 50,
        quantity: 3,
    };
    assert!(vehicle.validate().is_ok());
}

#[test]
fn test_invalid_length_negative() {
    let vehicle = Vehicle {
        length: -10,
        quantity: 1,
    };
    assert!(vehicle.validate().is_err());
}

#[test]
fn test_invalid_length_zero() {
    let vehicle = Vehicle {
        length: 0,
        quantity: 1,
    };
    assert!(vehicle.validate().is_err());
}

#[test]
fn test_invalid_length_not_multiple_of_10() {
    let vehicle = Vehicle {
        length: 15,
        quantity: 1,
    };
    assert!(vehicle.validate().is_err());
}

#[test]
fn test_invalid_quantity_zero() {
    let vehicle = Vehicle {
        length: 10,
        quantity: 0,
    };
    assert!(vehicle.validate().is_err());
}

#[test]
fn test_invalid_quantity_negative() {
    let vehicle = Vehicle {
        length: 10,
        quantity: -1,
    };
    assert!(vehicle.validate().is_err());
}

// SearchRequest validation tests
#[test]
fn test_valid_search_request() {
    let request = SearchRequest {
        vehicles: vec![
            Vehicle { length: 10, quantity: 2 },
            Vehicle { length: 20, quantity: 3 },
        ],
    };
    assert!(request.validate().is_ok());
}

#[test]
fn test_search_request_empty_vehicles() {
    let request = SearchRequest { vehicles: vec![] };
    assert!(request.validate().is_err());
}

#[test]
fn test_search_request_total_quantity_at_limit() {
    let request = SearchRequest {
        vehicles: vec![
            Vehicle { length: 10, quantity: 2 },
            Vehicle { length: 20, quantity: 3 },
        ],
    };
    let total: i32 = request.vehicles.iter().map(|v| v.quantity).sum();
    assert_eq!(total, 5);
    assert!(request.validate().is_ok());
}

#[test]
fn test_search_request_total_quantity_exceeds_limit() {
    let request = SearchRequest {
        vehicles: vec![
            Vehicle { length: 10, quantity: 3 },
            Vehicle { length: 20, quantity: 3 },
        ],
    };
    assert!(request.validate().is_err());
}

#[test]
fn test_search_request_invalid_vehicle_length() {
    let request = SearchRequest {
        vehicles: vec![Vehicle {
            length: 15,
            quantity: 1,
        }],
    };
    assert!(request.validate().is_err());
}

#[test]
fn test_search_request_invalid_vehicle_quantity() {
    let request = SearchRequest {
        vehicles: vec![Vehicle {
            length: 10,
            quantity: 0, 
        }],
    };
    assert!(request.validate().is_err());
}
