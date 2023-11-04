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
            vec3!(5.0, -5.0, -2.0),
            vec3!(5.0, 5.0, -2.0),
            vec3!(-5.0, 5.0, -2.0),
            Color::MAGENTA,
        )),
        Box::new(Trig::from(
            vec3!(5.0, -5.0, -2.0),
            vec3!(-5.0, -5.0, -2.0),
            vec3!(-5.0, 5.0, -2.0),
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
            if object.is_ray_intersect(ray) {
                intersecting.push(object);
            }
        }

        if intersecting.is_empty() {
            return SKY_COLOR.into();
        }

        let colliding = intersecting
            .into_iter()
            .map(|obj| (obj, obj.give_t(ray)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        match self.get_ray_brightness(colliding, ray) {
            None => SKY_COLOR.into(),
            Some(a) => {
                let c = a.min(255.0).max(0.0) / 255.0;
                (colliding.get_color() * c).into()
            }
        }
    }

    fn get_ray_brightness(&self, object: &Box<dyn Object3d + Sync>, ray: &Ray) -> Option<f32> {
        if !object.is_ray_intersect(ray) {
            return None;
        }

        let mut brightness: f32 = 0.;
        for light_source in &self.lights {
            let light_ray = Ray::from(
                ray.pos + ray.dir * (object.give_t(ray) - 0.001),
                vec3!() - (ray.pos + ray.dir * object.give_t(ray) - light_source.pos).normalize(),
            );

            let is_light_ray_intersect: bool = self
                .objects
                .iter()
                .any(|elem| elem.is_ray_intersect(&light_ray));

            if !is_light_ray_intersect {
                brightness += light_source.int
                    * (vec3!() - ray.dir)
                        .dot(light_source.pos - (ray.pos + ray.dir * object.give_t(ray)))
                    / (object.give_t(ray)
                        + (light_source.pos - (ray.pos + object.give_t(ray) * ray.dir)).length())
                    .powi(2)
            }
        }
        Some(brightness)
    }
}

