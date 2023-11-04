use euler::vec3;
use rayon::prelude::*;

use crate::color::*;
use crate::primitives::*;

use std::f32::consts::{FRAC_PI_4, SQRT_2};

const SKY_COLOR: Color = Color::SKYBLUE;

fn construct(t: f32) -> (Vec<Box<dyn Object3d + Sync>>, Vec<Light>) {
    let t = t + FRAC_PI_4;
    let tsin = t.sin();
    let tcos = t.cos();

    let mut vertex = [[[vec3!(); 2]; 2]; 2];
    vertex[0][0][0] = vec3!(-1.0, -SQRT_2 * tsin, -SQRT_2 * tcos);
    vertex[0][0][1] = vec3!(-1.0, -SQRT_2 * tcos, SQRT_2 * tsin);
    vertex[0][1][0] = vec3!(-1.0, SQRT_2 * tcos, -SQRT_2 * tsin);
    vertex[0][1][1] = vec3!(-1.0, SQRT_2 * tsin, SQRT_2 * tcos);
    vertex[1][0][0] = vec3!(1.0, -SQRT_2 * tsin, -SQRT_2 * tcos);
    vertex[1][0][1] = vec3!(1.0, -SQRT_2 * tcos, SQRT_2 * tsin);
    vertex[1][1][0] = vec3!(1.0, SQRT_2 * tcos, -SQRT_2 * tsin);
    vertex[1][1][1] = vec3!(1.0, SQRT_2 * tsin, SQRT_2 * tcos);

    let objects: Vec<Box<dyn Object3d + Sync>> = vec![
        Box::new(Trig::from(
            vertex[0][0][0],
            vertex[0][1][0],
            vertex[0][0][1],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[0][1][1],
            vertex[0][1][0],
            vertex[0][0][1],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[1][0][0],
            vertex[1][1][0],
            vertex[1][0][1],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[1][1][1],
            vertex[1][0][1],
            vertex[1][1][0],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[0][0][0],
            vertex[1][0][0],
            vertex[0][0][1],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[1][0][1],
            vertex[1][0][0],
            vertex[0][0][1],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[0][1][0],
            vertex[1][1][0],
            vertex[0][1][1],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[1][1][1],
            vertex[0][1][1],
            vertex[1][1][0],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[0][0][0],
            vertex[0][1][0],
            vertex[1][0][0],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[1][1][0],
            vertex[0][1][0],
            vertex[1][0][0],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[0][0][1],
            vertex[0][1][1],
            vertex[1][0][1],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vertex[1][1][1],
            vertex[1][0][1],
            vertex[0][1][1],
            Color::BLUE,
        )),
        Box::new(Trig::from(
            vec3!(5.0, -5.0, -0.5),
            vec3!(5.0, 5.0, -0.5),
            vec3!(-5.0, 5.0, -0.5),
            Color::MAGENTA,
        )),
        Box::new(Trig::from(
            vec3!(5.0, -5.0, -0.5),
            vec3!(-5.0, -5.0, -0.5),
            vec3!(-5.0, 5.0, -0.5),
            Color::MAGENTA,
        )),
    ];

    let lights = vec![Light::from(
        vec3!(2.0, 1.0, 10.0) / 2.0,
        10000.0,
        Color::ORANGE,
    )];

    (objects, lights)
}

pub struct Tracer {
    side: usize,
    camera: Camera,
    objects: Vec<Box<dyn Object3d + Sync>>,
    lights: Vec<Light>,
}

impl Tracer {
    pub fn from(side: usize, camera: Camera) -> Self {
        Self {
            side,
            camera,
            objects: vec![],
            lights: vec![],
        }
    }
    fn set_scene(&mut self, t: f32) {
        let (objects, lights) = construct(t);
        self.objects = objects;
        self.lights = lights;
    }
    pub fn draw(&mut self, t: f32, screen: &mut [u8]) {
        self.set_scene(t);

        let scr = self.side as f32 / 2.0;
        let screen_pre: Vec<_> = (0..self.side.pow(2))
            .into_par_iter()
            .map(|pos| {
                let (x, y) = (pos / self.side, pos % self.side);
                self.get_color((y as f32 - scr) / scr, (x as f32 - scr) / scr)
            })
            .collect();

        for (pos, pix) in screen.chunks_exact_mut(4).enumerate() {
            pix.copy_from_slice(&screen_pre[pos]);
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> [u8; 4] {
        let ray: Ray = self.camera.get_ray(u, v);
        self.cast_ray(&ray)
    }

    fn cast_ray(&self, ray: &Ray) -> [u8; 4] {
        let mut intersecting: Vec<&Box<dyn Object3d + Sync>> = Vec::new();

        for object in &self.objects {
            if object.intersects(ray) {
                intersecting.push(object);
            }
        }

        if intersecting.is_empty() {
            return SKY_COLOR.into();
        }

        let colliding = intersecting
            .into_iter()
            .map(|obj| (obj, obj.get_t(ray)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        let coef = self.get_ray_brightness(colliding, ray);
        let coef = coef.min(255.0).max(0.0) / 255.0;
        (colliding.get_color() * coef).into()
    }

    fn get_ray_brightness(&self, object: &Box<dyn Object3d + Sync>, ray: &Ray) -> f32 {
        self.lights.iter().fold(0.0, |acc, light_source| {
            let object_t = object.get_t(ray);
            let pos = ray.pos + ray.dir * (object_t - 0.001);
            let to_light = light_source.pos - pos;
            let light_ray = Ray::from(pos, to_light.normalize());

            let is_light_ray_intersect: bool =
                self.objects.iter().any(|elem| elem.intersects(&light_ray));

            if is_light_ray_intersect {
                acc
            } else {
                acc + light_source.int * (vec3!() - ray.dir).dot(to_light)
                    / (object.get_t(ray) + (to_light).length()).powi(2)
            }
        })
    }
}

