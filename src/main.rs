use euler::vec3;
use pixels::{Error, Pixels, SurfaceTexture};
use std::time::Instant;
use winit::{
    dpi::LogicalSize,
    event::{Event, VirtualKeyCode},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod color;
use color::*;

mod tracer;
use tracer::*;

const SIDE: usize = 512;
const TARGET_FPS: u64 = 60;

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

    let pos = vec3!(6.0, 0.0, 3.0);
    let camera = Camera::from(
        pos,
        vec3!() - pos.normalize(),
        vec3!(0.0, 1.0, 0.0),
        vec3!() - vec3!(-1.0, 0.0, 2.0).normalize(),
    );

    let mut t: f32 = 1.3;
    let mut tracer = Tracer::from(SIDE, camera, Box::new(|t| construct(t)));

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            window.request_redraw();

            let elapsed_time_f32 = Instant::now().duration_since(start_time).as_secs_f32();

            let elapsed_time = (elapsed_time_f32 * 1000.0) as u64;
            let wait_millis = match 1000 / TARGET_FPS >= elapsed_time {
                true => 1000 / TARGET_FPS - elapsed_time,
                false => 0,
            };
            let new_inst = start_time + std::time::Duration::from_millis(wait_millis);

            *control_flow = ControlFlow::WaitUntil(new_inst);
        }

        if let Event::RedrawRequested(_) = event {
            let start_draw = Instant::now();

            tracer.draw(t, pixels.frame_mut());
            t += 0.05;

            let draw_time = Instant::now().duration_since(start_draw).as_secs_f32();
            let fps = 1.0 / draw_time;
            println!("fps: {:.1} , draw time: {:.2} ms", fps, draw_time * 1000.0);

            if let Err(_err) = pixels.render() {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }
    });
}

use std::f32::consts::PI;

fn construct(t: f32) -> (Vec<Box<dyn Object3d>>, Vec<Light>) {
    let mut vertex = [[[vec3!(); 2]; 2]; 2];
    vertex[0][0][0] = vec3!(
        -1.0,
        -(2.0 as f32).sqrt() * (t + PI / 4.0).sin(),
        -(2.0 as f32).sqrt() * (t + PI / 4.0).cos()
    );
    vertex[0][0][1] = vec3!(
        -1.0,
        -(2.0 as f32).sqrt() * (t + PI / 4.0).cos(),
        (2.0 as f32).sqrt() * (t + PI / 4.0).sin()
    );
    vertex[0][1][0] = vec3!(
        -1.0,
        (2.0 as f32).sqrt() * (t + PI / 4.0).cos(),
        -(2.0 as f32).sqrt() * (t + PI / 4.0).sin()
    );
    vertex[0][1][1] = vec3!(
        -1.0,
        (2.0 as f32).sqrt() * (t + PI / 4.0).sin(),
        (2.0 as f32).sqrt() * (t + PI / 4.0).cos()
    );
    vertex[1][0][0] = vec3!(
        1.0,
        -(2.0 as f32).sqrt() * (t + PI / 4.0).sin(),
        -(2.0 as f32).sqrt() * (t + PI / 4.0).cos()
    );
    vertex[1][0][1] = vec3!(
        1.0,
        -(2.0 as f32).sqrt() * (t + PI / 4.0).cos(),
        (2.0 as f32).sqrt() * (t + PI / 4.0).sin()
    );
    vertex[1][1][0] = vec3!(
        1.0,
        (2.0 as f32).sqrt() * (t + PI / 4.0).cos(),
        -(2.0 as f32).sqrt() * (t + PI / 4.0).sin()
    );
    vertex[1][1][1] = vec3!(
        1.0,
        (2.0 as f32).sqrt() * (t + PI / 4.0).sin(),
        (2.0 as f32).sqrt() * (t + PI / 4.0).cos()
    );

    let objects: Vec<Box<dyn Object3d>> = vec![
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

    (objects.into(), lights)
}

