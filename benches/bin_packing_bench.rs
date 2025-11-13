use criterion::{black_box, criterion_group, criterion_main, Criterion};
use neighbor::bin_packing::search_locations;
use neighbor::model::{AllListings, Vehicle};

fn bench_api_search(c: &mut Criterion) {
    let listings = AllListings::get();
    let listings_slice = listings.inner();

    let mut group = c.benchmark_group("api_search");

    group.bench_function("single_vehicle", |b| {
        let vehicles = vec![Vehicle { length: 10, quantity: 1 }];
        b.iter(|| {
            search_locations(black_box(vehicles.clone()), black_box(listings_slice))
        });
    });

    group.bench_function("readme_example", |b| {
        let vehicles = vec![
            Vehicle { length: 10, quantity: 1 },
            Vehicle { length: 20, quantity: 2 },
            Vehicle { length: 25, quantity: 1 },
        ];
        b.iter(|| {
            search_locations(black_box(vehicles.clone()), black_box(listings_slice))
        });
    });

    group.bench_function("max_vehicles", |b| {
        let vehicles = vec![Vehicle { length: 50, quantity: 5 }];
        b.iter(|| {
            search_locations(black_box(vehicles.clone()), black_box(listings_slice))
        });
    });

    group.bench_function("mixed_sizes", |b| {
        let vehicles = vec![
            Vehicle { length: 30, quantity: 2 },
            Vehicle { length: 60, quantity: 2 },
            Vehicle { length: 100, quantity: 1 },
        ];
        b.iter(|| {
            search_locations(black_box(vehicles.clone()), black_box(listings_slice))
        });
    });

    group.finish();
}

criterion_group!(benches, bench_api_search);
criterion_main!(benches);
