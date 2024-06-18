use glam::Vec3;
use octree::octree::*;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use rand_distr::Normal;
use std::time::Instant;

fn add_n_points(n: i32) {
    let mut rng: rand::prelude::ThreadRng = thread_rng();
    let side = Normal::new(0.0f32, 1.0f32).unwrap();

    let mut points: Vec<Vec3> = Default::default();
    for _ in 0..n {
        let point = Vec3::new(rng.sample(side), rng.sample(side), rng.sample(side));
        points.push(point);
    }

    let now = Instant::now();
    Octree::<Vec3, EmptyNode>::fixed_depth(&points, 6);
    let elapsed = now.elapsed();

    println!("fixed depth {n} points => {elapsed:.3?}");

    let now = Instant::now();
    Octree::<Vec3, EmptyNode>::variable_depth(&points, 5);
    let elapsed = now.elapsed();

    println!("variable depth {n} points => {elapsed:.3?}");
}

fn main() {
    add_n_points(100);
    add_n_points(1_000);
    add_n_points(10_000);
    add_n_points(100_000);
    add_n_points(1_000_000);
    add_n_points(10_000_000);
}
