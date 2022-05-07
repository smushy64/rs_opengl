extern crate sdl2;
extern crate fmath;
extern crate gl;
extern crate wavefront_obj_importer;

pub mod resources;
pub mod shaders;
pub mod c_string;
pub mod opengl_fn;
pub mod input;
use input::Input;
pub mod transform;
use transform::Transform;

use gl::types::*;
use fmath::types::*;
use fmath::functions::angles::degrees_to_radians as d2r;

fn main() {

    resources::initialize();

    let sdl = sdl2::init().unwrap();

    let icon = resources::load_image("program/images/cube.png").unwrap();
    let mut icon_data = icon.to_rgba8().into_raw();

    let icon_surface = sdl2::surface::Surface::from_data(
        &mut icon_data,
        icon.width(), icon.height(),
        icon.width() * core::mem::size_of::<u32>() as u32,
        sdl2::pixels::PixelFormatEnum::RGBA32
    ).unwrap();

    let mut window = {
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(3, 3);
        video.window("OpenGL", 1280, 720)
            .opengl()
            .position_centered()
            .input_grabbed()
            .build().unwrap()
    };

    window.set_icon( &icon_surface );
    sdl.mouse().set_relative_mouse_mode(true);

    drop( icon_surface );
    drop( icon_data );
    drop( icon );

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

    let vertices:Vec<f32> = vec![
        // front
        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */  0.0, 0.0,  1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */  0.0, 0.0,  1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */  0.0, 0.0,  1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */  0.0, 0.0,  1.0, /* UVs */  1.0,  0.0,

        // back
        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */  0.0, 0.0, -1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5, -0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */  0.0, 0.0, -1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */  0.0, 0.0, -1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */  0.0, 0.0, -1.0, /* UVs */  1.0,  0.0,

        // left
        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */ -1.0, 0.0,  0.0, /* UVs */  0.0,  1.0,
        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */ -1.0, 0.0,  0.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */ -1.0, 0.0,  0.0, /* UVs */  0.0,  0.0,
        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */ -1.0, 0.0,  0.0, /* UVs */  1.0,  0.0,

        // right
        /* Positions */  0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */  1.0, 0.0,  0.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */  1.0, 0.0,  0.0, /* UVs */  1.0,  1.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */  1.0, 0.0,  0.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */  1.0, 0.0,  0.0, /* UVs */  1.0,  0.0,

        // top
        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */  0.0, 1.0,  0.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */  0.0, 1.0,  0.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */  0.0, 1.0,  0.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */  0.0, 1.0,  0.0, /* UVs */  1.0,  0.0,

        // bottom
        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */   0.0, -1.0, 0.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */   0.0, -1.0, 0.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, -1.0, 0.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */   0.0, -1.0, 0.0, /* UVs */  1.0,  0.0,
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

    let shader = resources::load_shader_program( "shaders/triangle" ).unwrap();

    let aspect_ratio:f32 = 1280.0 / 720.0;

    let _ortho_projection = opengl_fn::ortho(
        -1.6, 1.6,
        -0.9, 0.9,
        0.1, 100.0
    );

    let _persp_projection = opengl_fn::persp(
        d2r(90.0),
        aspect_ratio,
        0.1, 100.0
    );

    let cube_transform = Transform::new();

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

    let mut input = Input::new();

    let speed:f32 = 1.2;

    let mut last_elapsed:f32 = 0.0;
    let mut mouse = Vector2::new_zero();

    let mut camera_position = Vector3::new( 0.0, 0.0, 3.0 );
    let mut camera_front = Vector3::new_back();
    let camera_up = Vector3::new_up();

    let mut yaw   = -90.0;
    let mut pitch = 0.0;

    let mut running:bool = true;
    while running {

        use sdl2::event::Event;

        let elapsed = (_timer.ticks() as f32) / 1000.0;
        let delta_time = elapsed - last_elapsed;

        let last_mouse = mouse;

        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => { running = false; }
                Event::KeyDown { keycode: key, .. } => {
                    process_input(&mut input, key, true);
                },
                Event::KeyUp { keycode: key, .. } => {
                    process_input(&mut input, key, false);
                },
                Event::MouseMotion { xrel, yrel, .. } => {
                    mouse[0] += xrel as f32;
                    mouse[1] += yrel as f32;
                    input.set_mouse( mouse );
                }
                _ => {}
            }
        }

        if last_elapsed == 0.0 {
            mouse = Vector2::new_zero();
            input.set_mouse(mouse);
        }

        last_elapsed = elapsed;
        
        input.set_mouse_delta( mouse - last_mouse );

        {
            let camera_right = Vector3::cross( &camera_front, &camera_up ).normal();
            yaw   += input.mouse_delta().x() * delta_time *   10.0;
            pitch += -(input.mouse_delta().y() * delta_time * 10.0);
            pitch = pitch.clamp(-80.0, 80.0);

            camera_front = Vector3::new(
                d2r(yaw).cos() * d2r(pitch).cos(),
                d2r(pitch).sin(),
                d2r(yaw).sin() * d2r(pitch).cos()
            ).normal();

            if input.front != input.back {
                if input.front {
                    camera_position = camera_position + ( camera_front * speed * delta_time );
                } else if input.back {
                    camera_position = camera_position - ( camera_front * speed * delta_time );
                }
            }

            if input.right != input.left {
                if input.right {
                    camera_position = camera_position + ( camera_right * speed * delta_time );
                } else if input.left {
                    camera_position = camera_position - ( camera_right * speed * delta_time );
                }   
            }

            if input.up != input.down {
                if input.up {
                    camera_position = camera_position + ( camera_up * speed * delta_time );
                } else if input.down {
                    camera_position = camera_position - ( camera_up * speed * delta_time );
                }
            }

        }

        let look_at = opengl_fn::new_look_at_mat(
            &camera_position,
            &( camera_position + camera_front ),
            &camera_up
        );

        let proj_mat = {
            if input.is_ortho_enabled() {
                &_ortho_projection
            } else {
                &_persp_projection
            }
        };

        unsafe {

            gl::Clear( gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
            gl::UseProgram( shader.id() );
    
            gl::UniformMatrix4fv(
                view_id, 1, gl::FALSE,
                look_at.as_array().as_ptr()
            );
    
            gl::UniformMatrix4fv(
                projection_id, 1, gl::FALSE,
                proj_mat.as_array().as_ptr()
            );
            
            gl::UniformMatrix4fv(
                model_id, 1, gl::FALSE,
                cube_transform.mat_ptr()
            );
            
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

    drop( gl_ctx );
    drop( sdl );

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

use sdl2::keyboard::Keycode;
fn process_input( input:&mut Input, key_code:Option<Keycode>, is_down:bool ) {
    match key_code {
        Some(key) => {
            match key {
                Keycode::W => {
                    if is_down {
                        input.front = true;
                    } else {
                        input.front = false;
                    }
                },
                Keycode::A => {
                    if is_down {
                        input.left = true;
                    } else {
                        input.left = false;
                    }
                },
                Keycode::S => {
                    if is_down {
                        input.back = true;
                    } else {
                        input.back = false;
                    }
                },
                Keycode::D => {
                    if is_down {
                        input.right = true;
                    } else {
                        input.right = false;
                    }
                },
                Keycode::O => {
                    if is_down {
                        input.toggle_ortho();
                    }
                },
                Keycode::Space => {
                    if is_down {
                        input.up = true;
                    } else {
                        input.up = false;
                    }
                },
                Keycode::LShift => {
                    if is_down {
                        input.down = true;
                    } else {
                        input.down = false;
                    }
                },
                _ => {}
            }
        },
        None => {},
    }
}