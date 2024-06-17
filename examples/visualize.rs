use glam::{vec3, Vec3};
use macroquad::prelude as quad;
use macroquad::prelude::Color;
use octree::iterators::tree::TreeElements;
use octree::octree::Octree;
use rand::distributions::Uniform;
use rand::{thread_rng, Rng};
use rand_distr::Normal;

fn random_points(n: usize) -> Vec<Vec3> {
    let mut rng = thread_rng();
    let side = Normal::new(0.0f32, 1.0f32).unwrap();

    (0..n)
        .map(|_| vec3(rng.sample(side), rng.sample(side), 0.0))
        .collect()
}

#[macroquad::main("Octree")]
async fn main() {
    let points = random_points(1000);
    let tree = Octree::variable_depth(&points, 5);
    let scale = 80.0;
    println!("{tree:#?}");

    loop {
        quad::clear_background(quad::BLACK);

        for elem in &tree {
            match elem {
                TreeElements::Node { center, size } => {
                    quad::draw_rectangle_lines(
                        quad::screen_width() / 2.0 + scale * center.x - 0.5 * scale * size,
                        quad::screen_height() / 2.0 + scale * center.y - 0.5 * scale * size,
                        scale * size,
                        scale * size,
                        2.0,
                        quad::color_u8!(0, 0, 255, 63),
                    );
                }
                TreeElements::Point { point } => {
                    quad::draw_circle(
                        quad::screen_width() / 2.0 + scale * point.x,
                        quad::screen_height() / 2.0 + scale * point.y,
                        0.02 * scale,
                        quad::GREEN,
                    );
                }
            }
        }

        quad::draw_text("IT WORKS!", 20.0, 20.0, 30.0, quad::DARKGRAY);

        quad::next_frame().await
    }
}
