use crate::color::Color;
use crate::primitives::*;
use crate::utils::*;
use euler::vec3;

fn construct_camera() -> Camera {
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
}

