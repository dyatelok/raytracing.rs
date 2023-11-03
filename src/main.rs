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

/*fn construct(t: f32) -> (Vec<Object3d>, Vec<Light>) {
    let mut TT: Vec<Object3d> = vec![];

    let T: Object3d = Object3d::Trig(Trig::from(
        vec3!(5.0, -5.0, -1.0),
        vec3!(5.0, 5.0, -1.0),
        vec3!(-5.0, 5.0, -1.0),
        Color::MAGENTA,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vec3!(5.0, -5.0, -1.0),
        vec3!(-5.0, -5.0, -1.0),
        vec3!(-5.0, 5.0, -1.0),
        Color::MAGENTA,
    ));
    TT.push(T);

    let T: Object3d = Object3d::Sphere(Sphere::from(vec3!(), 1.0, Color::RED));
    TT.push(T);

    let mut LL: Vec<Light> = vec![];

    let L: Light = Light::from(vec3!(2.0, 1.0, 2.0) * 5.0, 7500.0, Color::ORANGE);
    LL.push(L);

    (TT, LL)
}*/

const SIDE: usize = 512;
const TARGET_FPS: u64 = 60;

fn draw(t: f32, screen: &mut [u8]) {
    let scr = SIDE as f32 / 2.0;
    let pos = vec3!(6.0, 0.0, 3.0);
    let cam: Camera = Camera::from(
        pos,
        vec3!() - pos.normalize(),
        vec3!(0.0, 1.0, 0.0),
        vec3!() - vec3!(-1.0, 0.0, 2.0).normalize(),
    );
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

    let mut t: f32 = 1.3;

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

            draw(t, pixels.frame_mut());
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

fn construct(t: f32) -> (Vec<Object3d>, Vec<Light>) {
    let pi = std::f32::consts::PI;
    let mut vertex = [[[vec3!(); 2]; 2]; 2];
    vertex[0][0][0] = vec3!(
        -1.0,
        -(2.0 as f32).sqrt() * (t + pi / 4.0).sin(),
        -(2.0 as f32).sqrt() * (t + pi / 4.0).cos()
    );
    vertex[0][0][1] = vec3!(
        -1.0,
        -(2.0 as f32).sqrt() * (t + pi / 4.0).cos(),
        (2.0 as f32).sqrt() * (t + pi / 4.0).sin()
    );
    vertex[0][1][0] = vec3!(
        -1.0,
        (2.0 as f32).sqrt() * (t + pi / 4.0).cos(),
        -(2.0 as f32).sqrt() * (t + pi / 4.0).sin()
    );
    vertex[0][1][1] = vec3!(
        -1.0,
        (2.0 as f32).sqrt() * (t + pi / 4.0).sin(),
        (2.0 as f32).sqrt() * (t + pi / 4.0).cos()
    );
    vertex[1][0][0] = vec3!(
        1.0,
        -(2.0 as f32).sqrt() * (t + pi / 4.0).sin(),
        -(2.0 as f32).sqrt() * (t + pi / 4.0).cos()
    );
    vertex[1][0][1] = vec3!(
        1.0,
        -(2.0 as f32).sqrt() * (t + pi / 4.0).cos(),
        (2.0 as f32).sqrt() * (t + pi / 4.0).sin()
    );
    vertex[1][1][0] = vec3!(
        1.0,
        (2.0 as f32).sqrt() * (t + pi / 4.0).cos(),
        -(2.0 as f32).sqrt() * (t + pi / 4.0).sin()
    );
    vertex[1][1][1] = vec3!(
        1.0,
        (2.0 as f32).sqrt() * (t + pi / 4.0).sin(),
        (2.0 as f32).sqrt() * (t + pi / 4.0).cos()
    );

    let mut TT: Vec<Object3d> = vec![];

    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[0][0][0],
        vertex[0][1][0],
        vertex[0][0][1],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[0][1][1],
        vertex[0][1][0],
        vertex[0][0][1],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[1][0][0],
        vertex[1][1][0],
        vertex[1][0][1],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[1][1][1],
        vertex[1][0][1],
        vertex[1][1][0],
        Color::BLUE,
    ));
    TT.push(T);

    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[0][0][0],
        vertex[1][0][0],
        vertex[0][0][1],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[1][0][1],
        vertex[1][0][0],
        vertex[0][0][1],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[0][1][0],
        vertex[1][1][0],
        vertex[0][1][1],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[1][1][1],
        vertex[0][1][1],
        vertex[1][1][0],
        Color::BLUE,
    ));
    TT.push(T);

    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[0][0][0],
        vertex[0][1][0],
        vertex[1][0][0],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[1][1][0],
        vertex[0][1][0],
        vertex[1][0][0],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[0][0][1],
        vertex[0][1][1],
        vertex[1][0][1],
        Color::BLUE,
    ));
    TT.push(T);
    let T: Object3d = Object3d::Trig(Trig::from(
        vertex[1][1][1],
        vertex[1][0][1],
        vertex[0][1][1],
        Color::BLUE,
    ));
    TT.push(T);

    let mut LL: Vec<Light> = vec![];

    let L: Light = Light::from(vec3!(2.0, 1.0, 10.0) / 2.0, 10000.0, Color::ORANGE);
    LL.push(L);

    (TT, LL)
}

