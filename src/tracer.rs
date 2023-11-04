use euler::{vec3, Vec3};

use super::color::*;

pub struct Tracer {
    side: usize,
    camera: Camera,
    objects: Vec<Box<dyn Object3d>>,
    lights: Vec<Light>,
    constructor: Box<dyn Fn(f32) -> (Vec<Box<dyn Object3d>>, Vec<Light>)>,
}

impl Tracer {
    pub fn from(
        side: usize,
        camera: Camera,
        constructor: Box<dyn Fn(f32) -> (Vec<Box<dyn Object3d>>, Vec<Light>)>,
    ) -> Self {
        Self {
            side,
            camera,
            objects: vec![],
            lights: vec![],
            constructor,
        }
    }
    fn set(&mut self, t: f32) {
        let (objects, lights) = (*self.constructor)(t);
        self.objects = objects;
        self.lights = lights;
    }
    pub fn draw(&mut self, t: f32, screen: &mut [u8]) {
        let scr = self.side as f32 / 2.0;
        self.set(t);

        for (pos, pix) in screen.chunks_exact_mut(4).enumerate() {
            let (x, y) = (pos / self.side, pos % self.side);
            let color = self.get_color((y as f32 - scr) / scr, (x as f32 - scr) / scr);
            pix.copy_from_slice(&color);
        }
    }

    fn cast_ray(&self, ray: &Ray) -> [u8; 4] {
        let sky_color = Color::SKYBLUE;
        let mut intersecting: Vec<&Box<dyn Object3d>> = Vec::new();
        for object in &self.objects {
            if object.is_ray_intersect(ray) {
                intersecting.push(object);
            }
        }

        if intersecting.is_empty() {
            return sky_color.into();
        }

        let mut mem = intersecting[0];
        let memt = intersecting[0].give_t(ray);
        for v in intersecting {
            if v.give_t(ray) < memt {
                mem = v;
            }
        }

        match self.get_ray_brightness(mem, ray) {
            None => sky_color.into(),
            Some(a) => {
                let c = a.min(255.0).max(0.0) / 255.0;
                (mem.get_color() * c).into()
            }
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> [u8; 4] {
        let ray: Ray = self.camera.get_ray(u, v);
        self.cast_ray(&ray)
    }
    fn get_ray_brightness(&self, object: &Box<dyn Object3d>, ray: &Ray) -> Option<f32> {
        if !object.is_ray_intersect(ray) {
            return None;
        }
        let mut br: f32 = 0.;
        for light_source in &self.lights {
            let light_ray = Ray {
                pos: ray.pos + ray.dir * (object.give_t(ray) - 0.001),
                dir: vec3!()
                    - (ray.pos + ray.dir * object.give_t(ray) - light_source.pos).normalize(),
            };
            let mut is_light_ray_intersect: bool = false;
            for elem in &self.objects {
                is_light_ray_intersect =
                    is_light_ray_intersect || elem.is_ray_intersect(&light_ray);
            }
            if !is_light_ray_intersect {
                br += light_source.int
                    * (vec3!() - ray.dir)
                        .dot(light_source.pos - (ray.pos + ray.dir * object.give_t(ray)))
                    / (object.give_t(ray)
                        + (light_source.pos - (ray.pos + object.give_t(ray) * ray.dir)).length())
                    / (object.give_t(ray)
                        + (light_source.pos - (ray.pos + object.give_t(ray) * ray.dir)).length());
            }
        }
        Some(br)
    }
}

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
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            pos: self.pos,
            dir: (self.dir + u * self.base1 + v * self.base2).normalize(),
        }
    }
}

pub struct Ray {
    pos: Vec3,
    dir: Vec3,
}

pub struct Light {
    pos: Vec3,
    int: f32,
    #[allow(dead_code)]
    col: Color,
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

