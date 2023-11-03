use euler::{vec3, Vec3};

struct Camera {
    pos: Vec3,
    dir: Vec3,
    base1: Vec3,
    base2: Vec3,
}

impl Camera {
    fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray {
            pos: self.pos,
            dir: (self.dir + u * self.base1 + v * self.base2).normalize(),
        }
    }
}

struct Ray {
    pos: Vec3,
    dir: Vec3,
}

struct Light {
    pos: Vec3,
    int: f32,
    col: Color,
}

enum Object3d {
    Sphere(Sphere),
    Trig(Trig),
}

impl Object3d {
    fn is_ray_intersect(&self, R: &Ray) -> bool {
        match &self {
            &Object3d::Sphere(s) => s.is_ray_intersect(R),
            &Object3d::Trig(s) => s.is_ray_intersect(R),
        }
    }
    fn give_t(&self, R: &Ray) -> f32 {
        match &self {
            &Object3d::Sphere(s) => s.give_t(R),
            &Object3d::Trig(s) => s.give_t(R),
        }
    }
    fn get_ray_brightness(&self, R: &Ray, L: &Vec<Light>, O: &Vec<Object3d>) -> Option<f32> {
        match &self {
            &Object3d::Sphere(s) => s.get_ray_brightness(R, L, O),
            &Object3d::Trig(s) => s.get_ray_brightness(R, L, O),
        }
    }
    fn get_color(&self) -> Color {
        match &self {
            &Object3d::Sphere(s) => s.get_color(),
            &Object3d::Trig(s) => s.get_color(),
        }
    }
}

struct Sphere {
    pos: Vec3,
    rad: f32,
    col: Color,
}

impl Sphere {
    fn is_ray_intersect(&self, R: &Ray) -> bool {
        let v: Vec3 = R.pos - self.pos;
        let b: f32 = 2.0 * v.dot(R.dir);
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

    fn give_t(&self, R: &Ray) -> f32 {
        let v: Vec3 = R.pos - self.pos;
        let b: f32 = 2.0 * v.dot(R.dir);
        let c: f32 = v.dot(v) - self.rad * self.rad;
        let d: f32 = b * b - 4.0 * c;
        if d < 0.0 {
            return -1.0;
        }
        let t0: f32 = (-b - d.sqrt()) / 2.0;
        let t1: f32 = (-b + d.sqrt()) / 2.0;
        t0.min(t1)
    }

    fn get_ray_brightness(&self, R: &Ray, L: &Vec<Light>, O: &Vec<Object3d>) -> Option<f32> {
        if self.is_ray_intersect(R) == false {
            return None;
        }
        let mut br: f32 = 0.;
        for l in L {
            let light_ray = Ray {
                pos: R.pos + R.dir * (self.give_t(R) - 0.001),
                dir: vec3!() - (R.pos + R.dir * self.give_t(R) - l.pos).normalize(),
            };
            let mut is_light_ray_intersect: bool = false;
            for o in O {
                is_light_ray_intersect = is_light_ray_intersect || o.is_ray_intersect(&light_ray);
            }
            if is_light_ray_intersect == false {
                br += l.int * (vec3!() - R.dir).dot(l.pos - (R.pos + R.dir * self.give_t(R)))
                    / (self.give_t(R) + (l.pos - (R.pos + self.give_t(R) * R.dir)).length())
                    / (self.give_t(R) + (l.pos - (R.pos + self.give_t(R) * R.dir)).length());
            }
        }
        Some(br)
    }
    fn get_color(&self) -> Color {
        self.col
    }
}

struct Trig {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    col: Color,
}

impl Trig {
    fn is_ray_intersect(&self, R: &Ray) -> bool {
        let norm: Vec3 = (self.v1 - self.v0).cross(self.v2 - self.v0);
        let A = norm.x;
        let B = norm.y;
        let C = norm.z;
        let D = -(A * self.v0.x + B * self.v0.y + C * self.v0.z);
        if A * R.dir.x + B * R.dir.y + C * R.dir.z == 0.0 {
            return false;
        }
        let t = -(D + A * R.pos.x + B * R.pos.y + C * R.pos.z)
            / (A * R.dir.x + B * R.dir.y + C * R.dir.z);
        if t < 0.0 {
            return false;
        }
        let M = R.pos + t * R.dir;
        let a = self.v0 - M;
        let b = self.v1 - M;
        let c = self.v2 - M;
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
    fn give_t(&self, R: &Ray) -> f32 {
        let norm: Vec3 = (self.v1 - self.v0).cross(self.v2 - self.v0);
        let A = norm.x;
        let B = norm.y;
        let C = norm.z;
        let D = -(A * self.v0.x + B * self.v0.y + C * self.v0.z);
        if A * R.dir.x + B * R.dir.y + C * R.dir.z == 0.0 {
            return -1.0;
        }
        let t = -(D + A * R.pos.x + B * R.pos.y + C * R.pos.z)
            / (A * R.dir.x + B * R.dir.y + C * R.dir.z);
        t
    }
    fn get_ray_brightness(&self, R: &Ray, L: &Vec<Light>, O: &Vec<Object3d>) -> Option<f32> {
        if self.is_ray_intersect(R) == false {
            return None;
        }
        let mut br: f32 = 0.;
        for l in L {
            let light_ray = Ray {
                pos: R.pos + R.dir * (self.give_t(R) - 0.001),
                dir: vec3!() - (R.pos + R.dir * self.give_t(R) - l.pos).normalize(),
            };
            let mut is_light_ray_intersect: bool = false;
            for o in O {
                is_light_ray_intersect = is_light_ray_intersect || o.is_ray_intersect(&light_ray);
            }
            if is_light_ray_intersect == false {
                br += l.int * (vec3!() - R.dir).dot(l.pos - (R.pos + R.dir * self.give_t(R)))
                    / (self.give_t(R) + (l.pos - (R.pos + self.give_t(R) * R.dir)).length())
                    / (self.give_t(R) + (l.pos - (R.pos + self.give_t(R) * R.dir)).length());
            }
        }
        Some(br)
    }
    fn get_color(&self) -> Color {
        self.col
    }
}

fn conv(col: Color) -> [u8; 4] {
    [col.r, col.g, col.b, col.a]
}

fn colorize(col: Color, k: f32) -> [u8; 4] {
    [
        (col.r as f32 * k) as u8,
        (col.g as f32 * k) as u8,
        (col.b as f32 * k) as u8,
        (col.a as f32 * k) as u8,
    ]
}

fn cast_ray(TT: &Vec<Object3d>, LL: &Vec<Light>, R: &Ray) -> [u8; 4] {
    let sky_color = Color::SKYBLUE;
    let mut V: Vec<&Object3d> = Vec::new();
    for t in TT {
        if t.is_ray_intersect(R) == true {
            V.push(t);
        }
    }
    let mut mem: &Object3d;
    let memt: f32;
    if V.len() == 0 {
        return conv(sky_color);
    }
    mem = V[0];
    memt = V[0].give_t(R);
    for v in V {
        if v.give_t(R) < memt {
            mem = v;
        }
    }

    match mem.get_ray_brightness(R, LL, TT) {
        None => conv(sky_color),
        Some(a) => {
            let c = a.min(255.0).max(0.0) / 255.0;
            colorize(mem.get_color(), c)
        }
    }
}

fn get_color(u: f32, v: f32, TT: &Vec<Object3d>, LL: &Vec<Light>, cam: &Camera) -> [u8; 4] {
    let R: Ray = cam.get_ray(u, v);
    cast_ray(TT, LL, &R)
}

fn construct(t: f32) -> (Vec<Object3d>, Vec<Light>) {
    let mut TT: Vec<Object3d> = vec![];
    let T: Object3d = Object3d::Trig(Trig {
        v0: vec3!(5.0, -5.0, -1.0),
        v1: vec3!(5.0, 5.0, -1.0),
        v2: vec3!(-5.0, 5.0, -1.0),
        col: Color::MAGENTA,
    });
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig {
        v0: vec3!(5.0, -5.0, -1.0),
        v1: vec3!(-5.0, -5.0, -1.0),
        v2: vec3!(-5.0, 5.0, -1.0),
        col: Color::MAGENTA,
    });
    TT.push(T);

    let T: Object3d = Object3d::Sphere(Sphere {
        pos: vec3!(),
        rad: 1.0,
        col: Color::RED,
    });
    TT.push(T);

    let mut LL: Vec<Light> = vec![];

    let L: Light = Light {
        pos: vec3!(2.0, 1.0, 2.0) * 5.0,
        int: 7500.0,
        col: Color::ORANGE,
    };
    LL.push(L);

    (TT, LL)
}

