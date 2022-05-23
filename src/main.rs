extern crate fmath;
extern crate sdl2;
extern crate gl;

pub mod resources;
pub mod shader;
pub mod c_string;
pub mod opengl_fn;
pub mod input;
pub mod light;
pub mod texture;
pub mod camera;
pub mod geometry;
pub mod time;
use time::Time;

use input::Input;
pub mod transform;

#[allow(unused_imports)]
use gl::types::*;

use fmath::types::*;
use fmath::functions::angles::degrees_to_radians as d2r;

use std::io::{ stdin, stdout, Write };

fn main() {

    resources::initialize();

    println!("1 - low poly sphere");
    println!("2 - smooth shaded low poly sphere");
    println!("3 - low poly suzanne");
    println!("4 - subdivided suzanne\n\n");
    print!("Make a selection: ");
    let _ = stdout().flush();
    let mut selection = String::new();
    stdin().read_line( &mut selection ).unwrap();
    let mesh_path = match selection.trim_end().parse::<u32>() {
        Ok(res) => {
            match res {
                1 => { "obj/sphere.obj" },
                2 => { "obj/smooth_sphere.obj" },
                3 => { "obj/suzanne.obj" },
                _ => { "obj/suzanne_hd.obj" }
            }
        },
        Err(_) => "obj/suzanne_hd.obj",
    };

    let sdl = sdl2::init().unwrap();

    // NOTE: Application Title
    let program_info = ProgramInfo {
        title: format!("OpenGL | Load .obj Demo"),
        dimensions: Vector2::new( 1280.0, 720.0 ),
    };

    let mut window = {
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(3, 3);
        video.window(
            &program_info.title,
            program_info.dimensions[0] as u32,
            program_info.dimensions[1] as u32
        )
            .opengl()
            .position_centered()
            .input_grabbed()
            .build().unwrap()
    };

    set_program_icon( "program/images/icon.png", &mut window );

    sdl.mouse().set_relative_mouse_mode( true );

    let gl_ctx = window.gl_create_context().unwrap();
    opengl_fn::load_functions( window.subsystem() );

    let mut event_pump = sdl.event_pump().unwrap();
    let sdl_timer = sdl.timer().unwrap();
    let mut timer = Time::new();

    // NOTE: clear color
    let clear_color = color::RGB::from_hex("#776094").unwrap();
    opengl_fn::set_clear_color( &clear_color );
    opengl_fn::set_viewport( &program_info.dimensions );

    let mut input = Input::new();
    // NOTE: Camera Speeds
    let slow_camera_speed:f32 = 1.5;
    let fast_camera_speed:f32 = 3.0;

    let mut camera = camera::Camera::new( camera::ProjectionMode::Perspective )
        .set_aspect_ratio( program_info.aspect_ratio() )
        .set_fov( d2r( 65.0 ) )
        .set_clipping_fields( 0.001, 100.0 )
        .build();

    *camera.transform.yaw_mut() = d2r(-90.0);

    let mesh = resources::load_mesh( mesh_path ).unwrap();

    let mesh_shader = resources::load_shader_program( "model" ).unwrap();
    mesh_shader.use_program();

    let mut mesh_transform = transform::Transform::new();
    mesh_transform.translate( &( Vector3::new_back() * 2.2 ) );

    let shader_transform_loc       = mesh_shader.get_uniform_location( "transform" );
    let shader_view_loc       = mesh_shader.get_uniform_location( "view" );
    let shader_camerapos_loc  = mesh_shader.get_uniform_location( "camera_position" );
    let shader_normal_mat_loc = mesh_shader.get_uniform_location( "normal_mat" );
    let mut normal_mat  = Matrix3x3::new_identity();
    mesh_shader.set_matrix3( shader_normal_mat_loc, &normal_mat );

    mesh_shader.set_matrix4( shader_transform_loc, mesh_transform.transform_mat() );
    mesh_shader.set_matrix4_by_name( "projection", camera.projection() );
    mesh_shader.set_vector3_by_name( "directional_light.direction", &-Vector3::new_one() );

    let mut light_color = color::HSV::new( 0.0, 0.0, 1.0 );

    mesh_shader.set_rgb_by_name( "directional_light.color", &light_color.as_rgb() );
    light_color.set_saturation( 0.2 );
    light_color.set_value( 0.2 );
    mesh_shader.set_rgb_by_name( "directional_light.ambient_color", &light_color.as_rgb() );

    let model_color = color::RGB::new_white();

    mesh_shader.set_rgb_by_name( "diffuse_color", &model_color );
    mesh_shader.set_f32_by_name( "specular_strength", &0.5 );
    mesh_shader.set_f32_by_name( "glossiness", &32.0 );

    let mut mouse = Vector2::new_zero();
    unsafe { gl::Enable( gl::DEPTH_TEST ); }
    let mut running:bool = true;
    while running {

        use sdl2::event::Event;

        timer.update( sdl_timer.ticks() );

        let last_mouse = mouse;

        for event in event_pump.poll_iter() {
            match event { 
                #[allow(unused_assignments)]
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

        running = !input.is_quitting();

        if timer.is_first_frame() {
            mouse.set( 0.0, 0.0 );
            input.set_mouse(mouse);
        }

        input.set_mouse_delta( mouse - last_mouse );

        *camera.transform.yaw_mut()   += d2r(input.mouse_delta().x() * timer.delta_time() * 10.0);
        *camera.transform.pitch_mut() -= d2r(input.mouse_delta().y() * timer.delta_time() * 10.0);
        *camera.transform.pitch_mut()  = camera.transform.pitch().clamp( d2r( -75.0 ), d2r( 75.0 ) );

        let up      = Vector3::new_up();
        let forward = camera.transform.forward();
        let right   = transform::Transform::calculate_right( &forward, &up );

        let input_direction = input.move_direction();
        
        let move_direction = {
            ( right   * input_direction[0] ) +
            ( up      * input_direction[1] ) +
            ( forward * input_direction[2] )
        }.normal();

        let move_speed = if input.speed_up { fast_camera_speed }
            else { slow_camera_speed };
        let translation = move_direction * move_speed * timer.delta_time();
        
        camera.transform.translate( &translation );

        mesh_transform.rotate( &Vector3::new( 0.0, d2r(timer.delta_time() * 10.0), 0.0 ) );

        normal_mat = Matrix4x4::transpose( mesh_transform.transform_mat().inverse().unwrap() ).as_matrix3x3();

        opengl_fn::clear_screen();

        mesh_shader.use_program();

        mesh_shader.set_matrix4( shader_view_loc, &camera.view() );
        mesh_shader.set_vector3( shader_camerapos_loc, camera.transform.get_position() );
        mesh_shader.set_matrix4( shader_transform_loc, mesh_transform.transform_mat() );
        mesh_shader.set_matrix3( shader_normal_mat_loc, &normal_mat );

        mesh.render();
        
        window.gl_swap_window();

    }

    drop( gl_ctx );
    drop( sdl );

}

pub struct ProgramInfo {
    pub title: String,
    pub dimensions: Vector2,
}

impl ProgramInfo {
    pub fn aspect_ratio( &self ) -> f32 {
        self.dimensions[0] / self.dimensions[1]
    }
}

fn set_program_icon( icon_path:&str, window:&mut sdl2::video::Window ) {
    let icon = resources::load_image( icon_path ).unwrap();
    let mut icon_data = icon.to_rgba8().into_raw();

    let icon_surface = sdl2::surface::Surface::from_data(
        &mut icon_data,
        icon.width(), icon.height(),
        icon.width() * core::mem::size_of::<u32>() as u32,
        sdl2::pixels::PixelFormatEnum::RGBA32
    ).unwrap();

    window.set_icon( &icon_surface );
}

use sdl2::keyboard::Keycode;
fn process_input( input:&mut Input, key_code:Option<Keycode>, is_down:bool ) {
    match key_code {
        Some(key) => {
            match key {
                Keycode::W => {
                    if is_down {
                        input.forward = true;
                    } else {
                        input.forward = false;
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
