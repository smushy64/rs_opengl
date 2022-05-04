extern crate sdl2;
extern crate fmath;
extern crate gl;
extern crate wavefront_obj_importer;

pub mod resources;
pub mod shaders;
pub mod c_string;
pub mod opengl_fn;

use gl::types::*;
use fmath::types::*;

fn main() {

    resources::initialize();

    let sdl = sdl2::init().unwrap();

    let window = {
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(3, 3);
        video.window("OpenGL", 1280, 720)
            .opengl()
            .position_centered()
            .build().unwrap()
    };

    let gl_ctx = window.gl_create_context().unwrap();
    gl::load_with(
        |symbol|
            window.subsystem().gl_get_proc_address(&symbol) as *const GLvoid
    );

    let mut event_pump = sdl.event_pump().unwrap();
    let _timer = sdl.timer().unwrap();

    let clear_color = color::RGB::from_hex("#e88bc9").unwrap();
    unsafe {
        gl::ClearColor(
            clear_color.r_f32(),
            clear_color.g_f32(),
            clear_color.b_f32(),
            1.0
        );
        gl::Viewport(0 as GLint, 0 as GLint, 1280, 720);
    }

    let mut running:bool = true;

    while running {
        use sdl2::event::Event;
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => { running = false; }
                _ => {}
            }
        }

        unsafe {

            gl::Clear( gl::COLOR_BUFFER_BIT );

        }

        window.gl_swap_window();

    }

    drop( sdl );
    drop( gl_ctx );

}
