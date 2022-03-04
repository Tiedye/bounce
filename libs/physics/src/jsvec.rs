use wasm_bindgen::prelude::*;

use glam::*;

#[wasm_bindgen]
pub struct JsVec2 {
    pub x: f32,
    pub y: f32,
}

impl JsVec2 {
    pub fn from(v: &Vec2) -> JsVec2 {
        JsVec2 { x: v.x, y: v.y }
    }
}
