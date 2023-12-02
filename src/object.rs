use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct Object {
    // pub name: String,
    pub x: f32, pub y: f32, pub z: f32,
    pub vx: f32, pub vy: f32, pub vz: f32,
    pub mass: f32,
}

impl Object {
    pub fn new(x: f32, y: f32, z: f32, vx: f32, vy: f32, vz: f32, mass: f32) -> Self {
        Object{x, y, z, vx, vy, vz, mass}
    }
}