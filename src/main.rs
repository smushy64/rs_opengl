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

    let vertices:Vec<f32> = vec![
        /* Positions */ -0.5, -0.5, 0.0, /* Color */ 1.0, 0.0, 0.0, /* Normals */  -1.0, 0.0, 0.0, /* UVs */  0.0, 0.0,
        /* Positions */  0.5, -0.5, 0.0, /* Color */ 0.0, 1.0, 0.0, /* Normals */   1.0, 0.0, 0.0, /* UVs */  0.0, 0.0,
        /* Positions */  0.0,  0.5, 0.0, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, 1.0, 0.0, /* UVs */  0.0, 0.0,
    ];

    let indeces:Vec<u32> = vec![ 0, 1, 2 ];

    let mut vbo:GLuint = 0;
    let mut vao:GLuint = 0;
    let mut ebo:GLuint = 0;

    // load triangle into gl
    unsafe {
        use core::mem::size_of;

        // vertex array object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // vertex buffer object
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer( gl::ARRAY_BUFFER, vbo );
        gl::BufferData(
            gl::ARRAY_BUFFER,
            ( vertices.len() * size_of::<f32>() ) as GLsizeiptr,
            vertices.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );

        // ebo
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            ( indeces.len() * size_of::<u32>() ) as GLsizeiptr,
            indeces.as_ptr() as *const GLvoid,
            gl::STATIC_DRAW
        );

        // vertex attrib pointer
        let stride = 11;
        // vertices
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            0, 3,
            gl::FLOAT, gl::FALSE,
            ( stride * size_of::<f32>() ) as GLsizei,
            0 as *const GLvoid
        );
        // vert colors
        gl::EnableVertexAttribArray(1);
        gl::VertexAttribPointer(
            1, 3,
            gl::FLOAT, gl::FALSE,
            ( stride * size_of::<f32>() ) as GLsizei,
            ( 3 * size_of::<f32>() ) as *const GLvoid
        );
        // normals
        gl::EnableVertexAttribArray(2);
        gl::VertexAttribPointer(
            2, 3,
            gl::FLOAT, gl::FALSE,
            ( stride * size_of::<f32>() ) as GLsizei,
            ( 6 * size_of::<f32>() ) as *const GLvoid
        );
        // texcoords
        gl::EnableVertexAttribArray(3);
        gl::VertexAttribPointer(
            3, 2,
            gl::FLOAT, gl::FALSE,
            ( stride * size_of::<f32>() ) as GLsizei,
            ( 9 * size_of::<f32>() ) as *const GLvoid
        );

        gl::BindVertexArray( 0 );
        gl::BindBuffer( gl::ARRAY_BUFFER, 0 );
    }

    // load shader
    let vert_src = resources::load_cstring("shaders/triangle.vert").unwrap();
    let frag_src = resources::load_cstring("shaders/triangle.frag").unwrap();

    let vert = shaders::Shader::vert_from_source( &vert_src ).unwrap();
    let frag = shaders::Shader::frag_from_source( &frag_src ).unwrap();

    let shader = shaders::ShaderProgram::from_shaders( &[vert, frag] ).unwrap();

    unsafe {
        gl::UseProgram( shader.id() );
    }

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

            gl::BindVertexArray( vao );
            gl::BindBuffer( gl::ARRAY_BUFFER, ebo );
            gl::DrawElements(
                gl::TRIANGLES,
                indeces.len() as GLint,
                gl::UNSIGNED_INT,
                core::ptr::null_mut() as *const GLvoid
            );

        }

        window.gl_swap_window();

    }

    drop( sdl );
    drop( gl_ctx );

}
