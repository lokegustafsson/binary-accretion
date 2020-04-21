use crate::constants::{COUNT, NEIGHBORS};
use crate::vector::{Float, Vector3};
use ordered_float::NotNan;
use std::collections::BinaryHeap;

pub fn nearest_neighbors(points: &[Vector3; COUNT]) -> Box<[[usize; NEIGHBORS]; COUNT]> {
    assert!(NEIGHBORS < COUNT);
    nearest_neighbors_quadratic(points)
}

// O(n^2 + n k log k) time, O(n k) space
fn nearest_neighbors_quadratic(points: &[Vector3; COUNT]) -> Box<[[usize; NEIGHBORS]; COUNT]> {
    let mut neighbors = Vec::with_capacity(NEIGHBORS * COUNT);
    for i in 0..COUNT {
        let mut surrounding: BinaryHeap<(NotNan<Float>, usize)> = BinaryHeap::new();
        for j in 0..COUNT {
            if i == j {
                continue;
            }
            let dist = (points[i] - points[j]).norm_squared();
            if surrounding.len() < NEIGHBORS {
                surrounding.push((NotNan::new(dist).unwrap(), j));
            } else if dist < *surrounding.peek().unwrap().0 {
                surrounding.pop();
                surrounding.push((NotNan::new(dist).unwrap(), j));
            }
        }
        assert_eq!(surrounding.len(), NEIGHBORS);
        let surrounding: Vec<usize> = surrounding
            .into_iter()
            .map(|(_dist, index)| index)
            .collect();
        for j in 0..NEIGHBORS {
            neighbors.push(surrounding[j]);
        }
    }
    unsafe {
        Box::from_raw(
            Box::into_raw(neighbors.into_boxed_slice()) as *mut [[usize; NEIGHBORS]; COUNT]
        )
    }
}
