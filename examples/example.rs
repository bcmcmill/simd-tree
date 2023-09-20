use simd_tree::*;
use std::time::Duration;
use std::time::Instant;

use generational_arena::Arena;
use rand::Rng;

fn main() {
    let num_points = 10_000_000;
    let mut rng = rand::thread_rng();
    let mut arena = Arena::new();

    let points: Vec<[f64; K]> =
        (0..num_points).fold(Vec::with_capacity(num_points), |mut vec, _| {
            vec.push([rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0)]);
            vec
        });

    let mut root = None;

    for point in points.iter() {
        root = Some(insert(&mut arena, root, *point));
    }

    let mut search_points = Vec::with_capacity(10_000);

    for _ in 0..10_000 {
        search_points.push(points[rng.gen_range(0..num_points - 1)]);
    }

    let mut total = Duration::ZERO;

    for search_point in search_points.iter() {
        let start_time = Instant::now();
        let _found = { search(&arena, root, *search_point) };
        let end_time = Instant::now();

        total += end_time - start_time;
    }

    println!("Avg search time: {:#?}", total / 10000);

    let mut total = Duration::ZERO;

    for search_point in search_points.iter() {
        let start_time = Instant::now();
        let _found = unsafe { search_simd(&arena, root, *search_point) };
        let end_time = Instant::now();

        total += end_time - start_time;
    }

    println!("Avg search_simd time: {:#?}", total / 10000);
}
