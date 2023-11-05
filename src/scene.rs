use crate::color::Color;
use crate::primitives::*;
use crate::utils::*;
use euler::vec3;

use std::f32::consts::FRAC_PI_3;

// pub const SKY_COLOR: Color = Color::SKYBLUE;
pub const SKY_COLOR: Color = Color::BLACK;
// pub const SKY_COLOR: Color = Color::WHITE;

fn construct_camera() -> Camera {
    let pos = vec3!(2.0, 2.0, 2.0) * 2.0;
    let dir = vec3![] - pos.normalize() * 1.0;
    let base0 = vec3![0.0, 0.0, 1.0];
    let base1 = vec3![] - base0.cross(dir).normalize();
    let base2 = dir.cross(base1).normalize();
    Camera::from(pos, dir, base1, base2)
}

fn construct_objects(_t: f32) -> Vec<Box<dyn Object3d + Sync>> {
    let t = 1.0;
    let t1 = t;
    let t2 = t + FRAC_PI_3 * 2.0;
    let t3 = t - FRAC_PI_3 * 2.0;

    let top = vec3![0.0, 0.0, 2.0 * 3_f32.sqrt()];
    let b1 = vec3![t1.sin() * 2.0, t1.cos() * 2.0, 0.0];
    let b2 = vec3![t2.sin() * 2.0, t2.cos() * 2.0, 0.0];
    let b3 = vec3![t3.sin() * 2.0, t3.cos() * 2.0, 0.0];

    vec![
        //spheres/lights
        Box::new(Sphere::from(
            vec3![3.0, -3.0, 0.7],
            0.7,
            Material::from(Color::GOLD, 0.0, 0.7, Color::ORANGE),
        )),
        Box::new(Sphere::from(
            vec3![-3.0, 3.0, 1.0],
            1.0,
            Material::from(Color::VIOLET, 0.0, 0.5, Color::LIME),
        )),
        //other spheres
        Box::new(Sphere::from(
            vec3![2.0, 2.5, -1.0],
            0.5,
            Material::from(Color::WHITE, 0.0, 0.0, Color::BLACK),
        )),
        Box::new(Sphere::from(
            vec3![1.0, 2.0, -0.5],
            1.0,
            Material::from(Color::SKYBLUE, 0.0, 0.5, Color::PURPLE),
        )),
        Box::new(Sphere::from(
            vec3![-1.5, 2.7, -0.7],
            0.9,
            Material::from(Color::PURPLE, 1.0, 0.0, Color::BLACK),
        )),
        Box::new(Sphere::from(
            vec3![-2.0, -4.0, -0.7],
            3.0,
            Material::from(Color::RAYWHITE, 1.0, 0.0, Color::BLACK),
        )),
        Box::new(Sphere::from(
            vec3![-10.0, -8.0, 5.0],
            5.0,
            Material::from(Color::RAYWHITE, 1.0, 0.0, Color::BLACK),
        )),
        //prism
        Box::new(Trig::from(
            top,
            b1,
            b2,
            Material::from(Color::WHITE, 0.7, 0.0, Color::BLACK),
        )),
        Box::new(Trig::from(
            top,
            b2,
            b3,
            Material::from(Color::WHITE, 0.7, 0.0, Color::BLACK),
        )),
        Box::new(Trig::from(
            top,
            b3,
            b1,
            Material::from(Color::WHITE, 0.7, 0.0, Color::BLACK),
        )),
        Box::new(Trig::from(
            b3,
            b2,
            b1,
            Material::from(Color::WHITE, 0.7, 0.0, Color::BLACK),
        )),
        //ground
        Box::new(Trig::from(
            vec3!(5.0, 5.0, -1.0),
            vec3!(5.0, -5.0, -1.0),
            vec3!(-5.0, -5.0, -1.0),
            Material::from(Color::PURPLE, 0.9, 0.0, Color::BLACK),
        )),
        Box::new(Trig::from(
            vec3!(-5.0, -5.0, -1.0),
            vec3!(-5.0, 5.0, -1.0),
            vec3!(5.0, 5.0, -1.0),
            Material::from(Color::PURPLE, 0.9, 0.0, Color::BLACK),
        )),
        //lights
        Box::new(Sphere::from(
            vec3![0.0, 0.0, 25.0],
            10.0,
            Material::from(Color::BLACK, 0.0, 1.0, Color::BLUE),
        )),
        Box::new(Sphere::from(
            vec3![25.0, -25.0, 0.0],
            25.0,
            Material::from(Color::BLACK, 0.0, 1.0, Color::GREEN),
        )),
        Box::new(Sphere::from(
            vec3![-25.0, 25.0, 0.0],
            25.0,
            Material::from(Color::BLACK, 0.0, 1.0, Color::RED),
        )),
    ]
}

pub fn construct_scene(t: f32) -> (Camera, Vec<Box<dyn Object3d + Sync>>) {
    (construct_camera(), construct_objects(t))
}

/*fn construct_camera() -> Camera {
    let pos = vec3!(2.4, 0.0, 12.0);
    let dir = vec3![] - pos.normalize();
    let base1 = vec3![0.0, 1.0, 0.0];
    let base2 = dir.cross(base1).normalize();
    Camera::from(pos, dir, base1, base2)
}

fn construct_objects() -> Vec<Box<dyn Object3d + Sync>> {
    vec![
        Box::new(Sphere::from(
            vec3![0.0, 0.0, -100.0],
            100.0,
            Material::from(Color::PURPLE, 0.3, 0.0, Color::BLACK),
        )),
        Box::new(Sphere::from(
            vec3![0.0, 0.0, 3.0],
            3.0,
            Material::from(Color::RED, 1.0, 0.0, Color::BLACK),
        )),
        Box::new(Sphere::from(
            vec3![5.0, -1.0, 2.0],
            2.0,
            Material::from(Color::GREEN, 1.0, 0.0, Color::BLACK),
        )),
        Box::new(Sphere::from(
            vec3![8.0, -1.5, 1.0],
            1.0,
            Material::from(Color::BLUE, 0.5, 0.0, Color::BLACK),
        )),
        Box::new(Sphere::from(
            vec3![100.0, -100.0, 0.0],
            100.0,
            Material::from(Color::BLACK, 0.0, 1.0, Color::WHITE),
        )),
    ]
}

pub fn construct_scene(_t: f32) -> (Camera, Vec<Box<dyn Object3d + Sync>>) {
    (construct_camera(), construct_objects())
}*/

