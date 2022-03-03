use wasm_bindgen::prelude::*;

use glam::*;

#[wasm_bindgen]
pub struct Polygon {
    points: Vec<Vec2>,
}

impl Polygon {
    fn contains(&self, p: &Vec2) -> bool {
        self.points
            .iter()
            .zip(self.points.iter().cycle().skip(1))
            .any(|(p1, p2)| {
                let edge = *p2 - *p1;
                let d = *p - *p1;
                edge.perp_dot(d) >= 0.0 && d.dot(edge) <= edge.length_squared()
            })
    }
}

#[wasm_bindgen]
pub fn make_polygon(v: Vec<f32>) -> Polygon {
    Polygon {
        points: v
            .iter()
            .step_by(2)
            .zip(v.iter().skip(1).step_by(2))
            .map(|(x, y)| vec2(*x, *y))
            .collect(),
    }
}

#[wasm_bindgen]
pub fn polygon_contains(poly: &Polygon, x: f32, y: f32) -> bool {
    poly.contains(&vec2(x, y))
}
