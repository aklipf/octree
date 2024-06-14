#![feature(test)]

extern crate test;

use glam::Vec3;
use octree::octree::Octree;
use test::Bencher;
use rand::{Rng, thread_rng};
use rand::distributions::Uniform;

fn add_n_points(b: &mut Bencher, n:i32){
    let mut rng = thread_rng();
    let side = Uniform::new(-1.0f32, 1.0f32);

    let mut points:Vec<Vec3>=Default::default();
    for _ in 0..n {
        let point = Vec3::new(rng.sample(side), rng.sample(side), rng.sample(side));
        points.push(point);
    }

    b.iter(|| {
        points.iter().fold(&mut Octree::<usize>::default(),|tree,p| {tree.add(*p);tree});
    });
}

#[bench]
fn add_100_points(b: &mut Bencher) {
    add_n_points(b, 100);
}

#[bench]
fn add_1k_points(b: &mut Bencher) {
    add_n_points(b, 1_000);
}

#[bench]
fn add_10k_points(b: &mut Bencher) {
    add_n_points(b, 10_000);
}

#[bench]
fn add_100k_points(b: &mut Bencher) {
    add_n_points(b, 100_000);
}

#[bench]
fn add_1m_points(b: &mut Bencher) {
    add_n_points(b, 1_000_000);
}
