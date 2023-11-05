use super::color::*;
use euler::{vec3, Vec3};

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

#[derive(Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub reflexivity: f32,
    // pub emitting: f32,
    // metallicity: f32,
}

impl Material {
    pub fn from(
        color: Color,
        reflexivity: f32, /* emitting: f32*/ /*, metallicity: f32*/
    ) -> Self {
        Self {
            color,
            reflexivity,
            // emitting, /*, metallicity */
        }
    }
}

pub trait Object3d {
    fn intersects(&self, ray: &Ray) -> bool;
    fn get_t(&self, ray: &Ray) -> f32;
    fn get_mat(&self) -> Material;
    fn get_next_ray(&self, ray: &Ray) -> Ray;
    fn get_norm(&self, pos: Vec3) -> Vec3;
}

pub struct Sphere {
    pos: Vec3,
    rad: f32,
    mat: Material,
}

impl Sphere {
    #[allow(dead_code)]
    pub fn from(pos: Vec3, rad: f32, mat: Material) -> Self {
        Self { pos, rad, mat }
    }
}

impl Object3d for Sphere {
    fn intersects(&self, ray: &Ray) -> bool {
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
    fn get_t(&self, ray: &Ray) -> f32 {
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
    fn get_mat(&self) -> Material {
        self.mat
    }
    fn get_next_ray(&self, ray: &Ray) -> Ray {
        let t = self.get_t(ray);
        let pos = ray.pos + (t - 0.001) * ray.dir;
        let norm = self.get_norm(pos);

        let reflexivity = self.get_mat().reflexivity;

        let reflection = ray.dir + 2.0 * ray.dir.dot(vec3![] - norm) * norm;

        let direction =
            (reflexivity * reflection + (1.0 - reflexivity) * random_norm(norm)).normalize();

        Ray::from(pos, direction)
    }
    fn get_norm(&self, pos: Vec3) -> Vec3 {
        (pos - self.pos).normalize()
    }
}

use rand::prelude::*;
use rand_distr::StandardNormal;

fn random_sphere() -> Vec3 {
    let mut rng = thread_rng();
    let x: f32 = rng.sample(StandardNormal);
    let y: f32 = rng.sample(StandardNormal);
    let z: f32 = rng.sample(StandardNormal);
    vec3![x, y, z].normalize()
}

fn random_norm(norm: Vec3) -> Vec3 {
    let vec = random_sphere();
    if norm.dot(vec) < 0f32 {
        vec3![] - vec
    } else {
        vec
    }
}

/*pub struct Trig {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    mat: Material,
}

impl Trig {
    pub fn from(v0: Vec3, v1: Vec3, v2: Vec3, mat: Material) -> Self {
        Self { v0, v1, v2, mat }
    }
}

impl Object3d for Trig {
    fn intersects(&self, ray: &Ray) -> bool {
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
    fn get_t(&self, ray: &Ray) -> f32 {
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
        self.mat.color
    }
}
*/

