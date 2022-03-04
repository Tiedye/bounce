#![feature(total_cmp)]

use wasm_bindgen::prelude::*;

use glam::*;

#[derive(PartialEq, PartialOrd)]
struct Orderedf32(f32);

impl Ord for Orderedf32 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl Eq for Orderedf32 {}

trait Drawable {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d);
}

#[wasm_bindgen]
pub struct JsVec2 {
    pub x: f32,
    pub y: f32,
}

impl JsVec2 {
    fn from(v: &Vec2) -> JsVec2 {
        JsVec2 { x: v.x, y: v.y }
    }
}

#[wasm_bindgen]
pub struct Polygon {
    points: Vec<Vec2>,
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

impl Drawable for Polygon {
    fn draw(&self, ctx: &web_sys::CanvasRenderingContext2d) {
        if let Some(start) = self.points.first() {
            ctx.begin_path();
            ctx.move_to(start.x.into(), start.y.into());
            for (_, &end) in self.edges() {
                ctx.line_to(end.x.into(), end.y.into())
            }
            ctx.close_path();
        }
    }
}

impl Polygon {
    fn edges(&self) -> impl Iterator<Item = (&Vec2, &Vec2)> {
        self.points.iter().zip(self.points.iter().cycle().skip(1))
    }
    fn intersection(&self, p: Vec2) -> Option<Vec2> {
        self.edges()
            .max_by_key(|(&p1, &p2)| {
                let edge = p2 - p1;
                let d = p - p1;
                Orderedf32(edge.perp_dot(d) / edge.length())
            })
            .map(|(&p1, &p2)| {
                let edge = p2 - p1;
                let d = p - p1;
                if edge.perp_dot(d) < 0.0 {
                    Some(d.project_onto(edge) + p1)
                } else {
                    None
                }
            })
            .flatten()
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
pub fn polygon_intersection(poly: &Polygon, x: f32, y: f32) -> Option<JsVec2> {
    poly.intersection(vec2(x, y)).map(|i| JsVec2::from(&i))
}

#[wasm_bindgen]
pub fn polygon_draw(poly: &Polygon, ctx: &web_sys::CanvasRenderingContext2d) {
    poly.draw(ctx)
}
