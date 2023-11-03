use euler::vec3;
use pixels::{Error, Pixels, SurfaceTexture};
use raylib::prelude::*;
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

use tracer::*;

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

/*fn main() {
    let pixel_size = 4;
    let screen = 256;
    let scr = screen as f32 / 2.0;
    let pos = vec3!(6.0, 0.0, 3.0);
    let cam: Camera = Camera {
        pos: pos,
        dir: vec3!() - pos.normalize(),
        base1: vec3!(0.0, 1.0, 0.0),
        base2: vec3!() - vec3!(-1.0, 0.0, 2.0).normalize(),
    };
    let mut t = 1.3;
    let mut TL;

        TL = construct(t);
        for i in 0..screen {
            for j in 0..screen {
                d.draw_rectangle(
                    i * pixel_size,
                    j * pixel_size,
                    pixel_size,
                    pixel_size,
                    Color::from(get_color(
                        (i as f32 - scr) / scr,
                        (j as f32 - scr) / scr,
                        &TL.0,
                        &TL.1,
                        &cam,
                    )),
                );
            }
        }
        t += 0.05;
    }
}*/

const SIDE: usize = 512;
const TARGET_FPS: u64 = 60;

fn draw(screen: &mut [u8]) {
    let scr = SIDE as f32 / 2.0;
    let pos = vec3!(6.0, 0.0, 3.0);
    let cam: Camera = Camera {
        pos,
        dir: vec3!() - pos.normalize(),
        base1: vec3!(0.0, 1.0, 0.0),
        base2: vec3!() - vec3!(-1.0, 0.0, 2.0).normalize(),
    };
    let mut t = 1.3;
    let TL = construct(t);

    for (pos, pix) in screen.chunks_exact_mut(4).enumerate() {
        let (x, y) = (pos / SIDE, pos % SIDE);
        let color = get_color(
            (y as f32 - scr) / scr,
            (x as f32 - scr) / scr,
            &TL.0,
            &TL.1,
            &cam,
        );
        pix.copy_from_slice(&color);
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(SIDE as f64, SIDE as f64);
        let scaled_size = LogicalSize::new(SIDE as f64, SIDE as f64);
        WindowBuilder::new()
            .with_title("ray tracing")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SIDE as u32, SIDE as u32, surface_texture)?
    };

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            /*if input.key_pressed(VirtualKeyCode::Space) {
                cells.update();
            }*/

            // let start_compute = Instant::now();

            // let compute_time = Instant::now().duration_since(start_compute).as_secs_f32();

            // let start_draw = Instant::now();

            window.request_redraw();

            // let draw_time = Instant::now().duration_since(start_draw).as_secs_f32();

            let elapsed_time_f32 = Instant::now().duration_since(start_time).as_secs_f32();

            // let fps = 1.0 / elapsed_time_f32;

            // println!(
            //     "fps: {:.1} , loop time: {:.2} ms , compute time: {:.2} ms , draw time: {:.2} ms",
            //     fps,
            //     elapsed_time_f32 * 1000.0,
            //     compute_time * 1000.0,
            //     draw_time * 1000.0
            // );

            let elapsed_time = (elapsed_time_f32 * 1000.0) as u64;
            let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                true => 1000 / TARGET_FPS - elapsed_time,
                false => 0,
            };
            let new_inst = start_time + std::time::Duration::from_millis(wait_millis);

            *control_flow = ControlFlow::WaitUntil(new_inst);
        }

        if let Event::RedrawRequested(_) = event {
            draw(pixels.frame_mut());
            if let Err(_err) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
    });
}

/*
fn construct(t : f32) -> (Vec<Object3d>,Vec<Light>) {
    let pi = std::f32::consts::PI; /*
    let mut vertex = vec!(vec!(vec!(vec3!(); 2); 2); 2);
    vertex[0][0][0] = vec3!(-1.0,   - (2.0 as f32).sqrt() * (t + pi / 4.0).sin(),   - (2.0 as f32).sqrt() * (t + pi / 4.0).cos());
    vertex[0][0][1] = vec3!(-1.0,   - (2.0 as f32).sqrt() * (t + pi / 4.0).cos(),     (2.0 as f32).sqrt() * (t + pi / 4.0).sin());
    vertex[0][1][0] = vec3!(-1.0,     (2.0 as f32).sqrt() * (t + pi / 4.0).cos(),   - (2.0 as f32).sqrt() * (t + pi / 4.0).sin());
    vertex[0][1][1] = vec3!(-1.0,     (2.0 as f32).sqrt() * (t + pi / 4.0).sin(),     (2.0 as f32).sqrt() * (t + pi / 4.0).cos());
    vertex[1][0][0] = vec3!( 1.0,   - (2.0 as f32).sqrt() * (t + pi / 4.0).sin(),   - (2.0 as f32).sqrt() * (t + pi / 4.0).cos());
    vertex[1][0][1] = vec3!( 1.0,   - (2.0 as f32).sqrt() * (t + pi / 4.0).cos(),     (2.0 as f32).sqrt() * (t + pi / 4.0).sin());
    vertex[1][1][0] = vec3!( 1.0,     (2.0 as f32).sqrt() * (t + pi / 4.0).cos(),   - (2.0 as f32).sqrt() * (t + pi / 4.0).sin());
    vertex[1][1][1] = vec3!( 1.0,     (2.0 as f32).sqrt() * (t + pi / 4.0).sin(),     (2.0 as f32).sqrt() * (t + pi / 4.0).cos());*/


    let mut TT : Vec<Object3d> = vec!();

    /*let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[0][0][0],
        v1: vertex[0][1][0],
        v2: vertex[0][0][1],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[0][1][1],
        v1: vertex[0][1][0],
        v2: vertex[0][0][1],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[1][0][0],
        v1: vertex[1][1][0],
        v2: vertex[1][0][1],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[1][1][1],
        v1: vertex[1][0][1],
        v2: vertex[1][1][0],
    });
    TT.push(T);


    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[0][0][0],
        v1: vertex[1][0][0],
        v2: vertex[0][0][1],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[1][0][1],
        v1: vertex[1][0][0],
        v2: vertex[0][0][1],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[0][1][0],
        v1: vertex[1][1][0],
        v2: vertex[0][1][1],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[1][1][1],
        v1: vertex[0][1][1],
        v2: vertex[1][1][0],
    });
    TT.push(T);


    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[0][0][0],
        v1: vertex[0][1][0],
        v2: vertex[1][0][0],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[1][1][0],
        v1: vertex[0][1][0],
        v2: vertex[1][0][0],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[0][0][1],
        v1: vertex[0][1][1],
        v2: vertex[1][0][1],
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vertex[1][1][1],
        v1: vertex[1][0][1],
        v2: vertex[0][1][1],
    });
    TT.push(T);*/
    /*let T : Object3d = Object3d::Trig(Trig {
        v0: vec3!( 5.0,-5.0,-0.0),
        v1: vec3!( 5.0, 5.0,-0.0),
        v2: vec3!(-5.0, 5.0,-0.0),
    });
    TT.push(T);
    let T : Object3d = Object3d::Trig(Trig {
        v0: vec3!( 5.0,-5.0,-0.0),
        v1: vec3!(-5.0,-5.0,-0.0),
        v2: vec3!(-5.0, 5.0,-0.0),
    });
    TT.push(T);*/

    let T : Object3d = Object3d::Sphere(Sphere {
        pos: vec3!(),
        rad : 1.0
    });
    TT.push(T);

    let T : Object3d = Object3d::Sphere(Sphere {
        pos: vec3!(),
        rad : 0.0
    });
    TT.push(T);

    /*let T : Object3d = Object3d::Trig(Trig {
        v0: vec3!(),
        v1: vec3!(),
        v2: vec3!(),
    });
    TT.push(T);*/

    let mut LL : Vec<Light> = vec!();

    let L: Light = Light {
        pos: vec3!(2.0, 1.0, 2.0) / 2.0,
        int: 2000.0,
        col : Color::ORANGE
    };
    LL.push(L);

    (TT,LL)
}
*/

