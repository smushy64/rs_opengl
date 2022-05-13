extern crate sdl2;
extern crate fmath;
extern crate gl;
extern crate wavefront_obj_importer;

pub mod resources;
pub mod shader;
pub mod c_string;
pub mod opengl_fn;
pub mod input;
pub mod mesh;

use input::Input;
pub mod transform;
use transform::Transform;

use gl::types::*;
use fmath::types::*;
use fmath::functions::angles::degrees_to_radians as d2r;

fn main() {

    resources::initialize();

    let sdl = sdl2::init().unwrap();

    let icon = resources::load_image("program/images/icon.png").unwrap();
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
    sdl.mouse().set_relative_mouse_mode( true );

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

    let clear_color = color::RGB::from_hex("#776094").unwrap();
    // let clear_color = color::RGB::from_hex("#000000").unwrap();
    opengl_fn::set_clear_color( &clear_color );
    unsafe {
        gl::Viewport(0 as GLint, 0 as GLint, 1280, 720);
    }

    let cube_mesh = mesh::generate_cube();

    let light_shader = resources::load_shader_program("shaders/light").unwrap();
    let cube_shader = resources::load_shader_program( "shaders/cube" ).unwrap();

    cube_shader.use_program();
    
    let aspect_ratio:f32 = 1280.0 / 720.0;

    let perspective_projection = opengl_fn::persp(
        d2r(80.0),
        aspect_ratio,
        0.01, 100.0
    );

    let mut cube_transform_0 = Transform::new();
    cube_transform_0.set_rotation(
        Vector3::new(
            d2r(  0.0 ),
            d2r( 25.0 ),
            d2r(  0.0 ),
        )
    );

    let mut cube_transform_1 = Transform::new();
    cube_transform_1.set_position( Vector3::new_down() );
    cube_transform_1.set_scale( Vector3::new( 100.0, 1.0, 100.0 ) );

    let mut light_transform = Transform::new();

    light_transform.set_position( Vector3::new(1.0, 1.2, 1.0) );
    light_transform.set_scale( Vector3::new_one() * 0.2 );

    let mut input = Input::new();

    let speed:f32 = 1.2;

    let mut last_elapsed:f32 = 0.0;
    let mut mouse = Vector2::new_zero();

    let mut camera_position = Vector3::new( 0.0, 0.0, 3.0 );
    let mut camera_front = Vector3::new_back();
    let camera_up = Vector3::new_up();

    let mut yaw   = -90.0;
    let mut pitch = 0.0;

    
    cube_shader.use_program();
    
    let light_color = color::HSV::new( 0.0, 0.0, 1.0 );
    let light_rgb = light_color.as_rgb();
    // let mut hue = 0.0;

    cube_shader.set_vec3("light.ambient", &(Vector3::new_one() * 0.2) );
    // light color
    cube_shader.set_color("light.diffuse", &light_rgb );
    cube_shader.set_vec3("light.specular", &Vector3::new( 1.0, 1.0, 1.0 ) );
    cube_shader.set_vec3("light.position", light_transform.position());

    cube_shader.set_color("material.ambient", &light_rgb);
    cube_shader.set_vec3("material.diffuse", &Vector3::new( 1.0, 1.0, 1.0 ));
    cube_shader.set_vec3("material.specular", &Vector3::new( 0.5, 0.5, 0.5 ) );
    cube_shader.set_float("material.shininess", 32.0);

    let mut light_color_bulb = light_color.clone();
    light_color_bulb.set_saturation(0.1);

    light_shader.use_program();
    light_shader.set_color("lightColor", &light_color_bulb.as_rgb());


    let mut running:bool = true;

    unsafe { gl::Enable( gl::DEPTH_TEST ); }

    while running {

        use sdl2::event::Event;

        let elapsed = (_timer.ticks() as f32) / 1000.0;
        let delta_time = elapsed - last_elapsed;

        let last_mouse = mouse;

        // light_color.set_hue( hue );
        // hue += delta_time * 10.0;

        // light_color_bulb = light_color.clone();
        // light_color_bulb.set_saturation(0.1);

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

        if input.is_quitting() {
            running = false;
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

        let view_mat = opengl_fn::new_look_at_mat(
            &camera_position,
            &( camera_position + camera_front ),
            &camera_up
        );

        unsafe {

            gl::Clear( gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
    
            cube_shader.use_program();
            cube_shader.set_mat4("view", &view_mat);
            cube_shader.set_mat4("projection", &perspective_projection);
            cube_shader.set_mat4("model", cube_transform_0.mat());
            cube_shader.set_vec3("view_position", &camera_position);

            // light_rgb = light_color.as_rgb();

            // cube_shader.set_color("light.diffuse", &light_rgb );
            // cube_shader.set_color("material.ambient", &light_rgb);
            
            cube_mesh.render();

            // draw platform ============================================

            // cube_shader.set_mat4("model", cube_transform_1.mat());
            // cube_mesh.render();

            // draw light ===============================================

            light_shader.use_program();

            // light_shader.set_color("lightColor", &light_color_bulb.as_rgb());

            light_shader.set_mat4("view", &view_mat);
            light_shader.set_mat4("projection", &perspective_projection);
            light_shader.set_mat4("model", light_transform.mat());

            cube_mesh.render();

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
                Keycode::Escape => {
                    if is_down {
                        input.quit_game();
                    }
                },
                _ => {}
            }
        },
        None => {},
    }
}