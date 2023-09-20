#![feature(stdsimd)]
use generational_arena::{Arena, Index};

use std::arch::x86_64::*;

pub const K: usize = 2;

pub struct Node {
    point: [f64; K],
    left: Option<Index>,
    right: Option<Index>,
}

impl Node {
    #[inline(always)]
    pub fn new(point: [f64; K]) -> Self {
        Node {
            point,
            left: None,
            right: None,
        }
    }
}

#[inline(always)]
pub fn insert_rec(
    arena: &mut Arena<Node>,
    node: Option<Index>,
    point: [f64; K],
    depth: u32,
) -> Index {
    match node {
        Some(node) => {
            let cd = (depth % (K as u32)) as usize;

            if point[cd] < arena[node].point[cd] {
                let left = insert_rec(arena, arena[node].left, point, depth + 1);
                arena[node].left = Some(left);
            } else {
                let right = insert_rec(arena, arena[node].right, point, depth + 1);
                arena[node].right = Some(right);
            }

            node
        }
        None => arena.insert(Node::new(point)),
    }
}

#[inline(always)]
pub fn insert(arena: &mut Arena<Node>, node: Option<Index>, point: [f64; K]) -> Index {
    insert_rec(arena, node, point, 0)
}

#[inline(always)]
pub fn are_points_same(point1: [f64; K], point2: [f64; K]) -> bool {
    point1.iter().zip(point2.iter()).all(|(&a, &b)| a == b)
}

#[inline(always)]
pub fn search_rec(arena: &Arena<Node>, node: Option<Index>, point: [f64; K], depth: u32) -> bool {
    match node {
        Some(node) => {
            if are_points_same(arena[node].point, point) {
                true
            } else {
                let cd = (depth % (K as u32)) as usize;
                if point[cd] < arena[node].point[cd] {
                    search_rec(arena, arena[node].left, point, depth + 1)
                } else {
                    search_rec(arena, arena[node].right, point, depth + 1)
                }
            }
        }
        None => false,
    }
}

#[inline(always)]
pub fn search(arena: &Arena<Node>, node: Option<Index>, point: [f64; K]) -> bool {
    search_rec(arena, node, point, 0)
}

#[inline(always)]
pub unsafe fn are_points_same_simd(point1: [f64; 2], point2: [f64; 2]) -> bool {
    let a = _mm256_loadu_pd(point1.as_ptr());
    let b = _mm256_loadu_pd(point2.as_ptr());
    let cmp = _mm256_cmp_pd(a, b, _CMP_EQ_OS);
    let result = _mm256_movemask_pd(cmp);

    result == 0b1111
}

#[inline(always)]
pub unsafe fn search_rec_simd(
    arena: &Arena<Node>,
    node: Option<Index>,
    point: [f64; 2],
    depth: u32,
) -> bool {
    match node {
        Some(node) => {
            if are_points_same_simd(arena[node].point, point) {
                true
            } else {
                let cd = (depth % (2 as u32)) as usize;
                if point[cd] < arena[node].point[cd] {
                    search_rec_simd(arena, arena[node].left, point, depth + 1)
                } else {
                    search_rec_simd(arena, arena[node].right, point, depth + 1)
                }
            }
        }
        None => false,
    }
}

#[inline(always)]
pub unsafe fn search_simd(arena: &Arena<Node>, node: Option<Index>, point: [f64; K]) -> bool {
    search_rec_simd(arena, node, point, 0)
}

#[cfg(test)]
mod benchmarks {
    use super::*;

    use rand::Rng;
    use std::time::Instant;

    #[test]
    fn bench_insert() {
        let num_points = 1_000_000;
        let mut rng = rand::thread_rng();
        let mut arena = Arena::new();

        let points: Vec<[f64; K]> = (0..num_points)
            .map(|_| [rng.gen_range(0.0..100.0), rng.gen_range(0.0..100.0)])
            .collect();

        let mut root = None;

        let start_time = Instant::now();
        for point in points.iter() {
            root = Some(insert(&mut arena, root, *point));
        }
        let end_time = Instant::now();

        println!(
            "Insertion of {} points took: {:?}",
            num_points,
            end_time - start_time
        );
    }

    #[test]
    fn bench_search() {
        let num_points = 1_000_000;
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

        let mut search_points = Vec::with_capacity(1000);

        for _ in 0..1_000 {
            search_points.push(points[rng.gen_range(0..num_points - 1)]);
        }

        let mut res = Vec::with_capacity(1000);

        for search_point in search_points.iter() {
            let start_time = Instant::now();
            let found = search(&arena, root, *search_point);
            let end_time = Instant::now();

            if found {
                res.push(format!(
                    "Found point {:?} in {:?}",
                    search_point,
                    end_time - start_time
                ));
            } else {
                res.push(format!(
                    "Did not find point {:?} in {:?}",
                    search_point,
                    end_time - start_time
                ));
            }
        }

        println!("{}", res.join("\n"));
    }
}
