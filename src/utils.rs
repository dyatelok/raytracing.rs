use crate::color::Color;
use euler::{vec3, Vec3};

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    base1: Vec3,
    base2: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            pos: vec3![],
            dir: vec3![1.0, 0.0, 0.0],
            base1: vec3![0.0, 1.0, 0.0],
            base2: vec3![0.0, 0.0, 1.0],
        }
    }
    pub fn from(pos: Vec3, dir: Vec3, base1: Vec3, base2: Vec3) -> Self {
        Self {
            pos,
            dir,
            base1,
            base2,
        }
    }
    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            pos: self.pos,
            dir: (self.dir + u * self.base1 + v * self.base2).normalize(),
        }
    }
}

pub struct Ray {
    pub pos: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn from(pos: Vec3, dir: Vec3) -> Self {
        Self { pos, dir }
    }
}

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub metallicity: f32,
    pub emitting: f32,
    pub emitting_color: Color,
}

impl Material {
    pub fn from(color: Color, metallicity: f32, emitting: f32, emitting_color: Color) -> Self {
        Self {
            color,
            metallicity,
            emitting,
            emitting_color,
        }
    }
}

