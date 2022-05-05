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
use rand::Rng;

use rand::thread_rng;

fn main() {

    let mut rng = thread_rng();

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
        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  0.0,

        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */   0.0, 0.0, -1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5, -0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */   0.0, 0.0, -1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, 0.0, -1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */   0.0, 0.0, -1.0, /* UVs */  1.0,  0.0,

        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  1.0,
        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  0.0,
        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  0.0,

        /* Positions */  0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  1.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  0.0,

        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  0.0,

        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */   0.0, 0.0,  1.0, /* UVs */  1.0,  0.0,
    ];

    let indeces:Vec<u32> = vec![
        0, 1, 2,
        1, 3, 2,

        4, 5, 6,
        5, 7, 6,

        8,  9, 10,
        9, 11, 10,

        12, 13, 14,
        13, 15, 14,

        16, 17, 18,
        17, 19, 18,

        20, 21, 22,
        21, 23, 22,
    ];

    // load texture
    let texture = resources::load_image("textures/container.jpg").unwrap();

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

        gl::Enable( gl::DEPTH_TEST );
    }

    // load shader
    let vert_src = resources::load_cstring("shaders/triangle.vert").unwrap();
    let frag_src = resources::load_cstring("shaders/triangle.frag").unwrap();

    let vert = shaders::Shader::vert_from_source( &vert_src ).unwrap();
    let frag = shaders::Shader::frag_from_source( &frag_src ).unwrap();

    let shader = shaders::ShaderProgram::from_shaders( &[vert, frag] ).unwrap();

    let aspect_ratio:f32 = 1280.0 / 720.0;

    let _ortho_projection = opengl_fn::ortho(
        -1.6, 1.6,
        -0.9, 0.9,
        0.1, 1000.0
    );

    let _persp_projection = opengl_fn::persp(
        45.0,
        aspect_ratio,
        0.1, 100.0
    );

    use fmath::functions::angles::degrees_to_radians as d2r;
    let translate = Vector3::new( 0.0, 0.0, 0.0 );
    let rotation = Vector3::new( d2r( 0.0 ), 0.0, 0.0 );
    let scale = Vector3::new_one() * 1.0;
    #[allow(unused_mut)]
    let mut model_mat = Matrix4x4::new_trs(
        translate.as_array(),
        rotation.as_array(),
        scale.as_array()
    );

    let camera_translate = Vector3::new( 0.0, 0.0, -3.0 );
    let view_mat = Matrix4x4::new_translate( camera_translate.as_array() );

    // model's transform
    let model_id = shader.get_uniform_location( "model" );
    // camera's transform
    let view_id = shader.get_uniform_location( "view" );
    // projection matrix
    let projection_id = shader.get_uniform_location( "projection" );

    // load texture into shader
    unsafe {
        let mut tex_id:GLuint = 0;
        gl::GenTextures( 1, &mut tex_id );
        gl::BindTexture(gl::TEXTURE_2D, tex_id);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);

        let data = texture.to_rgb8().into_raw();

        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGB as GLint,
            texture.width() as GLint, texture.height() as GLint,
            0, gl::RGB, gl::UNSIGNED_BYTE,
            data.as_ptr() as *const GLvoid
        );

        drop( texture );
    }

    let mut models:Vec<Matrix4x4> = {

        let mut result = Vec::new();

        let count = 10;
        let mut counter = 0;
        while counter < count {

            let t = [
                rng.gen_range(-5.0f32..5.0f32),
                rng.gen_range(-5.0f32..5.0f32),
                rng.gen_range(-10.0f32..-2.0f32)
            ];

            let r = [
                rng.gen_range(-1.0f32..1.0f32),
                rng.gen_range(-1.0f32..1.0f32),
                rng.gen_range(-1.0f32..1.0f32),
            ];

            let s = [ 1.0, 1.0, 1.0 ];

            result.push( Matrix4x4::new_trs( &t, &r, &s, ) );

            counter += 1;
        }

        result

    };

    while running {
        use sdl2::event::Event;
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => { running = false; }
                _ => {}
            }
        }

        unsafe {

            gl::Clear( gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
            gl::UseProgram( shader.id() );
    
            gl::UniformMatrix4fv(
                view_id, 1, gl::FALSE,
                view_mat.as_array().as_ptr()
            );
    
            gl::UniformMatrix4fv(
                projection_id, 1, gl::FALSE,
                _persp_projection.as_array().as_ptr()
            );

            gl::BindVertexArray( vao );
            gl::BindBuffer( gl::ARRAY_BUFFER, ebo );

            render_cube( &model_mat, model_id, indeces.len() );

            for model in models.iter() {
                render_cube( model, model_id, indeces.len() );
            }

        }

        window.gl_swap_window();

        // model_mat = model_mat * Matrix4x4::new_translate( &[0.0, 0.0, 0.002] );
        model_mat = model_mat * Matrix4x4::new_rotate( &[0.001, 0.003, 0.002] );

        for model in models.iter_mut() {

            *model = *model * Matrix4x4::new_translate( &[ 0.0, -0.002, 0.0 ] );

        }

    }

    drop( sdl );
    drop( gl_ctx );

}

pub fn render_cube(
    model_matrix:&Matrix4x4,
    model_id:GLint,
    index_count:usize
) {

    unsafe {
        gl::UniformMatrix4fv(
            model_id, 1, gl::FALSE,
            model_matrix.as_array().as_ptr()
        );

        gl::DrawElements(
            gl::TRIANGLES,
            index_count as GLint,
            gl::UNSIGNED_INT,
            core::ptr::null_mut() as *const GLvoid
        );
    }

}
