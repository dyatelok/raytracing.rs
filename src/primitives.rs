use super::color::*;
use euler::Vec3;

pub struct Camera {
    pos: Vec3,
    dir: Vec3,
    base1: Vec3,
    base2: Vec3,
}

impl Camera {
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

pub struct Light {
    pub pos: Vec3,
    pub int: f32, //intensity
    #[allow(dead_code)]
    pub col: Color,
}

impl Light {
    pub fn from(pos: Vec3, int: f32, col: Color) -> Self {
        Self { pos, int, col }
    }
}

pub trait Object3d {
    fn is_ray_intersect(&self, ray: &Ray) -> bool;
    fn give_t(&self, ray: &Ray) -> f32;
    fn get_color(&self) -> Color;
}

pub struct Sphere {
    pos: Vec3,
    rad: f32,
    col: Color,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn from(pos: Vec3, rad: f32, col: Color) -> Self {
        Self { pos, rad, col }
    }
}

impl Object3d for Sphere {
    fn is_ray_intersect(&self, ray: &Ray) -> bool {
        let v: Vec3 = ray.pos - self.pos;
        let b: f32 = 2.0 * v.dot(ray.dir);
        let c: f32 = v.dot(v) - self.rad * self.rad;
        let d: f32 = b * b - 4.0 * c;
        if d < 0.0 {
            return false;
        }
        let t0: f32 = (-b - d.sqrt()) / 2.0;
        let t1: f32 = (-b + d.sqrt()) / 2.0;
        if t0 > 0.0 || t1 > 0.0 {
            return true;
        }
        false
    }
    fn give_t(&self, ray: &Ray) -> f32 {
        let v: Vec3 = ray.pos - self.pos;
        let b: f32 = 2.0 * v.dot(ray.dir);
        let c: f32 = v.dot(v) - self.rad * self.rad;
        let d: f32 = b * b - 4.0 * c;
        if d < 0.0 {
            return -1.0;
        }
        let t0: f32 = (-b - d.sqrt()) / 2.0;
        let t1: f32 = (-b + d.sqrt()) / 2.0;
        t0.min(t1)
    }
    fn get_color(&self) -> Color {
        self.col
    }
}

pub struct Trig {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    col: Color,
}

impl Trig {
    pub fn from(v0: Vec3, v1: Vec3, v2: Vec3, col: Color) -> Self {
        Self { v0, v1, v2, col }
    }
}

impl Object3d for Trig {
    fn is_ray_intersect(&self, ray: &Ray) -> bool {
        let norm: Vec3 = (self.v1 - self.v0).cross(self.v2 - self.v0);
        let a = norm.x;
        let b = norm.y;
        let c = norm.z;
        let d = -(a * self.v0.x + b * self.v0.y + c * self.v0.z);
        if norm.dot(ray.dir) == 0.0 {
            return false;
        }
        let t = -(d + a * ray.pos.x + b * ray.pos.y + c * ray.pos.z)
            / (a * ray.dir.x + b * ray.dir.y + c * ray.dir.z);
        if t < 0.0 {
            return false;
        }
        let m = ray.pos + t * ray.dir;
        let a = self.v0 - m;
        let b = self.v1 - m;
        let c = self.v2 - m;
        let base = (self.v2 - self.v0).cross(self.v1 - self.v0).normalize();
        if (a.cross(b).normalize() + base).length() > 0.01 {
            return false;
        }
        if (b.cross(c).normalize() + base).length() > 0.01 {
            return false;
        }
        if (c.cross(a).normalize() + base).length() > 0.01 {
            return false;
        }
        true
    }
    fn give_t(&self, ray: &Ray) -> f32 {
        let norm: Vec3 = (self.v1 - self.v0).cross(self.v2 - self.v0);
        let a = norm.x;
        let b = norm.y;
        let c = norm.z;
        let d = -(a * self.v0.x + b * self.v0.y + c * self.v0.z);
        if norm.dot(ray.dir) == 0.0 {
            return -1.0;
        }
        let t = -(d + a * ray.pos.x + b * ray.pos.y + c * ray.pos.z)
            / (a * ray.dir.x + b * ray.dir.y + c * ray.dir.z);
        t
    }
    fn get_color(&self) -> Color {
        self.col
    }
}

