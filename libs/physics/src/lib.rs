use wasm_bindgen::prelude::*;

use glam::*;

enum Collider {
    Point,
    Ellipse(Vec2),
    Rectangle(Vec2),
    ConvexPolygon(Vec<Vec2>),
}

struct Collision {
    colliders: Vec<Collider>,
}

struct Motion {
    velocity: Vec2,
    angular_velocity: f32,
}
struct Physics {
    mass: f32,
    moment_of_inertia: f32,
}

struct Position {
    pub pos: Vec2,
    pub rot: f32,
}

struct Object {}

#[wasm_bindgen]
pub fn start() {
    let mut objects: Vec<Object> = Vec::new();
    // loop
    loop {}
}
