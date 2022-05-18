extern crate sdl2;
extern crate fmath;
extern crate gl;
extern crate wavefront_obj_importer;
extern crate rand;

use rand::{thread_rng, Rng};

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

    // let clear_color = color::RGB::from_hex("#776094").unwrap();
    let clear_color = color::RGB::from_hex("#000000").unwrap();
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

    let mut rng = thread_rng();

    let cube_count = 10;
    let mut cube_transforms:Vec<Transform> = Vec::with_capacity( cube_count );
    let mut counter = 0;

    let xy_range = 5.0;
    let z_range  = 20.0;
    let rot_range = 180.0;

    while counter < cube_count {
        cube_transforms.push( Transform::new() );

        cube_transforms[counter].set_position(
            Vector3::new(
                rng.gen_range( -xy_range..xy_range ),
                rng.gen_range( -xy_range..xy_range ),
                rng.gen_range( -z_range..0.0 )
            )
        );
        cube_transforms[counter].set_rotation(
            Vector3::new(
                rng.gen_range( -rot_range..rot_range ),
                rng.gen_range( -rot_range..rot_range ),
                rng.gen_range( -rot_range..rot_range ),
            )
        );
        counter += 1;
    }

    let mut light_transform = Transform::new();
    let light_position_y = 1.2;

    light_transform.set_position( Vector3::new(1.0, light_position_y, 1.0) );
    light_transform.set_scale( Vector3::new_one() * 0.2 );

    let mut input = Input::new();
    // NOTE: Camera Speed
    let slow_camera_speed:f32 = 1.5;
    let fast_camera_speed:f32 = 3.0;

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

    cube_shader.set_vector3_by_name("light.ambient", &(Vector3::new_one() * 0.2) );
    // light color
    cube_shader.set_rgb_by_name("light.diffuse", &light_rgb );
    cube_shader.set_vector3_by_name("light.specular", &Vector3::new( 1.0, 1.0, 1.0 ) );
    let cube_shader_light_position_location = cube_shader.get_uniform_location("light.position");

    // loading diffuse and specular textures
    unsafe {

        let diffuse_texture =
            resources::load_image("textures/container2.png").unwrap();

        let specular_texture = 
            resources::load_image( "textures/container2_specular.png" ).unwrap();
        // let specular_texture = 
        //     resources::load_image( "textures/container2_specular_colored.png" ).unwrap();

        let mut diffuse_id = 0;
        gl::GenTextures( 1, &mut diffuse_id );
        gl::BindTexture( gl::TEXTURE_2D, diffuse_id );

        gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint );
        gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint );
        gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint );
        gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint );

        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGB as GLint,
            diffuse_texture.width() as GLint, diffuse_texture.height() as GLint,
            0, gl::RGB, gl::UNSIGNED_BYTE,
            diffuse_texture.to_rgb8().as_raw().as_ptr() as *const GLvoid
        );
        gl::GenerateMipmap( gl::TEXTURE_2D );

        let mut specular_id = 0;
        gl::GenTextures( 1, &mut specular_id );
        gl::BindTexture( gl::TEXTURE_2D, specular_id );

        gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint );
        gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint );
        gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint );
        gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint );

        gl::TexImage2D(
            gl::TEXTURE_2D, 0, gl::RGB as GLint,
            specular_texture.width() as GLint, specular_texture.height() as GLint,
            0, gl::RGB, gl::UNSIGNED_BYTE,
            specular_texture.to_rgb8().as_raw().as_ptr() as *const GLvoid
        );
        gl::GenerateMipmap( gl::TEXTURE_2D );

        gl::ActiveTexture( gl::TEXTURE0 );
        gl::BindTexture( gl::TEXTURE_2D, diffuse_id );
        gl::ActiveTexture( gl::TEXTURE1 );
        gl::BindTexture( gl::TEXTURE_2D, specular_id );

    }

    cube_shader.set_i32_by_name( "material.diffuse", &0 );  // use texcoord0
    cube_shader.set_i32_by_name( "material.specular", &1 ); // use texcoord1
    cube_shader.set_f32_by_name( "material.shininess", &16.0);

    cube_shader.set_f32_by_name( "light.constant",  &1.0   );
    cube_shader.set_f32_by_name( "light.linear",    &0.09  );
    cube_shader.set_f32_by_name( "light.quadratic", &0.032 );

    light_shader.use_program();
    let mut light_color = color::HSV::new( 0.0, 0.0, 1.0 );
    light_shader.set_rgb_by_name("lightColor", &light_color.as_rgb() );

    unsafe { gl::Enable( gl::DEPTH_TEST ); }

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

            let camera_speed = if input.speed_up {
                fast_camera_speed
            } else {
                slow_camera_speed
            };

            if input.front != input.back {
                if input.front {
                    camera_position = camera_position + ( camera_front * camera_speed * delta_time );
                } else if input.back {
                    camera_position = camera_position - ( camera_front * camera_speed * delta_time );
                }
            }

            if input.right != input.left {
                if input.right {
                    camera_position = camera_position + ( camera_right * camera_speed * delta_time );
                } else if input.left {
                    camera_position = camera_position - ( camera_right * camera_speed * delta_time );
                }   
            }

            if input.up != input.down {
                if input.up {
                    camera_position = camera_position + ( camera_up * camera_speed * delta_time );
                } else if input.down {
                    camera_position = camera_position - ( camera_up * camera_speed * delta_time );
                }
            }

        }

        let view_mat = opengl_fn::new_look_at_mat(
            &camera_position,
            &( camera_position + camera_front ),
            &camera_up
        );

        let mut new_light_position = light_transform.position().clone();
        new_light_position[1] = light_position_y + ( elapsed.sin() * 2.0 );
        light_transform.set_position( new_light_position );

        unsafe {
            gl::Clear( gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT );
        }
    
        cube_shader.use_program();

        let light_strength = elapsed.cos() + 2.0;
        light_color = color::HSV::new( 0.0, 0.0, light_strength / 3.0 );

        cube_shader.set_f32_by_name( "light.strength",  &light_strength );

        cube_shader.set_matrix4_by_name("view", &view_mat);
        cube_shader.set_matrix4_by_name("projection", &perspective_projection);
        cube_shader.set_vector3_by_name("view_position", &camera_position);

        cube_shader.set_vector4(
            cube_shader_light_position_location,
            &light_transform.position().as_vector4()
        );
        
        for cube_transform in cube_transforms.iter_mut() {
            render_mesh( &cube_mesh, &cube_shader, cube_transform.mat() );

            cube_transform.set_rotation(
                *cube_transform.rotation() +
                ( Vector3::new( 0.5, 0.3, -0.1 ) * delta_time )
            );

        }

        // draw light ===============================================

        light_shader.use_program();

        light_shader.set_matrix4_by_name("view", &view_mat);
        light_shader.set_matrix4_by_name("projection", &perspective_projection);

        light_shader.set_rgb_by_name("lightColor", &light_color.as_rgb() );
        
        render_mesh( &cube_mesh, &light_shader, light_transform.mat() );


        window.gl_swap_window();

    }

    drop( gl_ctx );
    drop( sdl );

}

fn render_mesh( mesh:&mesh::Mesh, shader:&shader::ShaderProgram, model_mat:&Matrix4x4 ) {

    shader.set_matrix4_by_name( "model", model_mat );
    mesh.render();

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
                        input.speed_up = true;
                    } else {
                        input.speed_up = false;
                    }
                },
                Keycode::LCtrl => {
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