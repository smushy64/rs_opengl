extern crate fmath;
extern crate sdl2;
extern crate gl;

pub mod resources;
pub mod c_string;
pub mod input;
pub mod light;
pub mod camera;
pub mod time;
pub mod transform;
pub mod graphics;

pub use time::Time;
pub use input::Input;
pub use transform::Transform;

use graphics::{ Mesh, Model, Material };
use fmath::{ types::*, functions::angles::degrees_to_radians as d2r, };

use std::io::{ stdin, stdout, Write };
pub use std::rc::Rc;

fn main() {

    resources::initialize();

    // let mesh_path = _select_mesh();
    let mesh_path = "space_ship.obj";

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
    graphics::load_glfn( window.subsystem() );

    let mut event_pump = sdl.event_pump().unwrap();
    let sdl_timer = sdl.timer().unwrap();
    let mut timer = Time::new();

    // NOTE: clear color
    let clear_color = color::RGB::from_hex("#444975").unwrap();
    graphics::clear_color( &clear_color );
    graphics::set_viewport( &program_info.dimensions );

    let mut input = Input::new();
    // NOTE: Camera Speeds
    let slow_camera_speed:f32 = 1.5;
    let fast_camera_speed:f32 = 3.0;

    let mut camera = camera::Camera::new( camera::ProjectionMode::Perspective )
        .set_aspect_ratio( program_info.aspect_ratio() )
        .set_fov( d2r( 65.0 ) )
        .set_clipping_fields( 0.001, 100.0 )
        .set_rotation( Vector3::new( 0.0, d2r( -90.0 ), 0.0 ) )
    .build();

    // NOTE: Mesh is loaded here!
    let mut mesh_transform = Transform::new(
        Vector3::new_back() * 5.0,
        Vector3::new( d2r(-10.0), d2r(100.0), d2r(60.0) ),
        Vector3::new_one()
    );
    let meshes = resources::load_meshes( mesh_path ).unwrap();
    let shader = resources::load_shader_program( "model" ).unwrap();
    let material = Material::new( "Ship", shader.clone() );
    let mut model = Model::new( meshes.clone(), material );
    model.material.activate_shader();

    let texture_options = graphics::texture::TextureOptions::default(); 

    let diffuse_texture =
        resources::load_texture( "space_ship_diff.png", texture_options ).unwrap();
    let specular_texture =
        resources::load_texture( "space_ship_spec.png", texture_options ).unwrap();

    let transform_loc = model.material.get_uniform_location( "transform" ).unwrap();
    let view_loc      = model.material.get_uniform_location( "view" ).unwrap();
    let normal_loc    = model.material.get_uniform_location( "normal_mat" ).unwrap();

    let camera_pos_loc = model.material.get_uniform_location( "camera_position" ).unwrap();

    model.material[view_loc].set_matrix4( camera.view() );
    model.material["projection"].set_matrix4( *camera.projection() );

    model.material[normal_loc].set_matrix3( *mesh_transform.normal_matrix() );

    model.material["directional_light.direction"].set_vector3(
        -( Vector3::new_up() + ( Vector3::new_forward() * 2.0 ) )
    );
    model.material["directional_light.color"].set_rgb( color::RGB::new_white() );
    model.material["directional_light.ambient_color"].set_rgb( color::RGB::new_white() * 0.12 );

    model.material[camera_pos_loc].set_vector3( *camera.transform.position() );
    model.material["glossiness"].set_f32( 32.0 );
    model.material["diffuse_texture"].set_texture( (
        diffuse_texture.clone(),
        graphics::texture::Sampler::new( 0 )
    ) );

    model.material["specular_texture"].set_texture( (
        specular_texture.clone(),
        graphics::texture::Sampler::new( 1 )
    ) );

    let mut mouse = Vector2::new_zero();
    let mouse_sens = 10.0;
    unsafe { gl::Enable( gl::DEPTH_TEST ); }
    let mut running:bool = true;
    while running {

        // UPDATE -------------------------------------------------------------------------------

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

        camera.transform.rotate( &(
            Vector3::new(
                d2r( -input.mouse_delta()[1] ),
                d2r(  input.mouse_delta()[0] ),
                0.0
            ) * timer.delta_time() * mouse_sens
        ) );
        camera.transform.rotation_mut()[0] = camera.transform.rotation()[0].clamp( d2r( -75.0 ), d2r( 75.0 ) );
        camera.transform.update_basis_vectors();

        let input_direction = input.move_direction();

        let move_direction = {
            ( *camera.transform.right() * input_direction[0] ) +
            ( *camera.transform.up() * input_direction[1] ) +
            ( *camera.transform.forward() * input_direction[2] )
        }.normal();

        let move_speed = if input.speed_up { fast_camera_speed }
            else { slow_camera_speed };
        let translation = move_direction * move_speed * timer.delta_time();
        
        camera.transform.translate( &translation );

        mesh_transform.rotate( &Vector3::new( 0.0, d2r(timer.delta_time() * 5.0), 0.0 ) );
        mesh_transform.update_transform_matrix();
        mesh_transform.update_normal_matrix();

        // RENDER -------------------------------------------------------------------------------
        graphics::clear_screen(); {

            model.material[transform_loc].set_matrix4( *mesh_transform.transform_matrix() );
            model.material[view_loc].set_matrix4( camera.view() );
            model.material[normal_loc].set_matrix3( *mesh_transform.normal_matrix() );
            model.material[camera_pos_loc].set_vector3( *camera.transform.position() );
    
            model.render();

        } window.gl_swap_window();

    }

    // clean up
    {
        drop( model );
        unsafe { graphics::texture::delete_textures( vec![ diffuse_texture, specular_texture ] ) };
        drop( gl_ctx );
        drop( sdl );
    }

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

fn _select_mesh() -> String {
    println!("1 - cube");
    println!("2 - sphere");
    println!("3 - cone");
    println!("4 - cylinder");
    println!("5 - icosphere");
    println!("6 - torus");
    println!("7 - suzanne");
    println!("\n\n");
    print!("Select a mesh to load: ");
    let _ = stdout().flush();
    let mut selection = String::new();
    stdin().read_line( &mut selection ).unwrap();
    let mesh_path = match selection.trim_end().parse::<u32>() {
        Ok(res) => {
            match res {
                2 => { "sphere.obj" },
                3 => { "cone.obj" },
                4 => { "cylinder.obj" },
                5 => { "icosphere.obj" },
                6 => { "torus.obj" },
                7 => { "suzanne.obj" }
                _ => { "cube.obj" },
            }
        },
        Err(_) => "cube.obj",
    };
    format!("{}", mesh_path)
}
