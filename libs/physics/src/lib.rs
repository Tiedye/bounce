#![feature(total_cmp)]

use wasm_bindgen::prelude::*;

use glam::*;

mod jsvec;
mod of32;

trait Drawable {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d);
}

#[wasm_bindgen]
pub struct Polygon {
    edges: Vec<(Vec2, Vec2)>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Drawable for Polygon {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        if let Some((start, _)) = self.edges.first() {
            ctx.begin_path();
            ctx.move_to(start.x.into(), start.y.into());
            for (_, end) in self.edges.iter() {
                ctx.line_to(end.x.into(), end.y.into())
            }
            ctx.close_path();
        }
    }
}

impl Polygon {
    fn from_points(points: &Vec<Vec2>) -> Polygon {
        Polygon {
            edges: points
                .iter()
                .zip(points.iter().cycle().skip(1))
                .map(|(&a, &b)| (a, b))
                .collect(),
        }
    }
    fn intersection(&self, p: Vec2) -> Option<Vec2> {
        self.edges
            .iter()
            .max_by_key(|(p1, p2)| {
                let edge = *p2 - *p1;
                let d = p - *p1;
                of32::Orderedf32(edge.perp_dot(d) / edge.length())
            })
            .and_then(|(p1, p2)| {
                let edge = *p2 - *p1;
                let d = p - *p1;
                if edge.perp_dot(d) < 0.0 {
                    Some(d.project_onto(edge) + *p1)
                } else {
                    None
                }
            })
    }
}

#[wasm_bindgen]
pub fn make_polygon(v: Vec<f32>) -> Polygon {
    Polygon::from_points(
        &v.iter()
            .step_by(2)
            .zip(v.iter().skip(1).step_by(2))
            .map(|(x, y)| vec2(*x, *y))
            .collect(),
    )
}

#[wasm_bindgen]
pub fn polygon_intersection(poly: &Polygon, x: f32, y: f32) -> Option<jsvec::JsVec2> {
    poly.intersection(vec2(x, y))
        .map(|i| jsvec::JsVec2::from(&i))
}

#[wasm_bindgen]
pub fn polygon_draw(poly: &Polygon, ctx: &web_sys::CanvasRenderingContext2d) {
    poly.draw(ctx)
}
