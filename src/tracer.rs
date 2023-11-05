use euler::vec3;
use rayon::prelude::*;

use crate::color::*;
use crate::primitives::*;
use crate::scene::construct_scene;
use crate::utils::*;

// const SKY_COLOR: Color = Color::SKYBLUE;
const SKY_COLOR: Color = Color::BLACK;

pub struct Tracer {
    side: usize,
    camera: Camera,
    objects: Vec<Box<dyn Object3d + Sync>>,
}

impl Tracer {
    pub fn from(side: usize) -> Self {
        Self {
            side,
            camera: Camera::new(),
            objects: vec![],
        }
    }
    fn set_scene(&mut self, t: f32) {
        let (camera, objects) = construct_scene(t);
        self.camera = camera;
        self.objects = objects;
    }
    pub fn draw(&mut self, t: f32, screen: &mut [u8]) {
        self.set_scene(t);

        let scr = self.side as f32 / 2.0;
        let screen_pre: Vec<_> = (0..self.side.pow(2))
            .into_par_iter()
            .map(|pos| {
                let (x, y) = (pos / self.side, pos % self.side);
                self.get_pixel_color((y as f32 - scr) / scr, (x as f32 - scr) / scr)
                    .into_u8()
            })
            .collect();

        for (pos, pix) in screen.chunks_exact_mut(4).enumerate() {
            pix.copy_from_slice(&screen_pre[pos]);
        }
    }

    pub fn get_pixel_color(&self, u: f32, v: f32) -> Color {
        let ray: Ray = self.camera.get_ray(u, v);
        const REFLECTION_LIMIT: usize = 5;
        self.cast_ray(&ray, REFLECTION_LIMIT)
    }

    fn cast_ray(&self, ray: &Ray, reflections: usize) -> Color {
        let mut intersecting: Vec<&Box<dyn Object3d + Sync>> = Vec::new();
        for object in &self.objects {
            if object.intersects(ray) {
                intersecting.push(object);
            }
        }

        if intersecting.is_empty() {
            return SKY_COLOR;
        }

        let to_collide = intersecting
            .into_iter()
            .map(|obj| (obj, obj.get_t(ray)))
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .unwrap()
            .0;

        match reflections {
            0 => {
                let object_t = to_collide.get_t(ray);
                let pos = ray.pos + ray.dir * (object_t - 0.001);
                let norm = to_collide.get_norm(pos);
                let light_direction = vec3![0.1, 0.1, -1.0];
                let to_light = vec3![] - light_direction;
                let light_ray = Ray::from(pos, to_light.normalize());
                let ray_intersect: bool =
                    self.objects.iter().any(|elem| elem.intersects(&light_ray));
                let sky_color = if ray_intersect {
                    Color::BLACK
                } else {
                    SKY_COLOR * to_collide.get_mat().color * norm.dot(to_light)
                };
                let emmiting_color =
                    to_collide.get_mat().emitting_color * to_collide.get_mat().emitting;

                sky_color + emmiting_color
            }
            refl => {
                let next_ray = to_collide.get_next_ray(ray);
                let coming = self.cast_ray(&next_ray, refl - 1);
                let coming_color = coming * to_collide.get_mat().color;
                let emmiting_color =
                    to_collide.get_mat().emitting_color * to_collide.get_mat().emitting;

                coming_color + emmiting_color
            }
        }
    }
}

