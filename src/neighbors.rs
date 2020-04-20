use crate::vector::{Float, Vector3};
use ordered_float::NotNan;
use std::collections::BinaryHeap;

pub fn nearest_neighbors(points: Vec<Vector3>, k: usize) -> Vec<Vec<usize>> {
    nearest_neighbors_quadratic(points, k)
}

// O(n^2 + n k log k) time, O(n k) space
fn nearest_neighbors_quadratic(points: Vec<Vector3>, k: usize) -> Vec<Vec<usize>> {
    let n = points.len();
    let mut neighbors: Vec<BinaryHeap<(NotNan<Float>, usize)>> = vec![BinaryHeap::new(); n];
    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let dist = (points[i] - points[j]).norm_squared();
            if neighbors[i].len() < k {
                neighbors[i].push((NotNan::new(dist).unwrap(), j));
            } else if dist < *neighbors[i].peek().unwrap().0 {
                neighbors[i].pop();
                neighbors[i].push((NotNan::new(dist).unwrap(), j));
            }
        }
    }
    neighbors
        .into_iter()
        .map(|heap| heap.into_iter().map(|pair| pair.1).collect())
        .collect()
}
