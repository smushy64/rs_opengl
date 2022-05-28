extern crate fmath;
extern crate sdl2;
extern crate gl;
pub use std::rc::Rc;
use debugging::print_start;

#[allow(unused_imports)]
use gl::types::*;
use texture::Sampler;
#[allow(unused_imports)]
use core::mem::size_of;
#[allow(unused_imports)]
use debugging::log;

pub mod resources;
pub mod cstr;
pub mod input;
pub mod time;
pub mod transform;
pub mod graphics;
pub mod debugging;

pub use time::Time;
pub use input::Input;
pub use transform::Transform;
use graphics::{ Camera, camera, Material, texture };


#[allow(unused_imports)]
use graphics::Mesh;
use fmath::types::*;

fn main() {

    resources::initialize();
    let sdl = sdl2::init().unwrap();
    // NOTE: Application Title
    let program_info = resources::load_program_info().unwrap();

    let mut window = {
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(4, 2);
        gl_attr.set_stencil_size( 8 );
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

    let _gl_ctx = window.gl_create_context().unwrap();
    graphics::load_glfn( window.subsystem() );

    print_start();
    // NOTE: Debug severity
    debugging::set_severity( debugging::GLDebugSeverity::High );

    let mut event_pump = sdl.event_pump().unwrap();
    let sdl_timer = sdl.timer().unwrap();
    let mut timer = Time::new();

    // NOTE: clear color
    let clear_color = color::RGB::from_hex("#a98bc4").unwrap();
    // let clear_color = color::RGB::new_black();
    graphics::set_clear_color( &clear_color );
    graphics::update_viewport( &program_info.dimensions );

    let mut input = Input::new();
    // Camera Speeds
    let slow_camera_speed:f32 = 1.5;
    let fast_camera_speed:f32 = 3.0;
    // mouse input buffer
    let mut mouse    = Vector2::new_zero();
    let mouse_sensitivity= 10.0;

    let mut camera = Camera::new(
        Vector3::new( 0.0, 0.2, 0.0 ),
        Vector3::new( 0.0, -90.0f32.to_radians(), 0.0 ),
        camera::Projection::perspective_default(),
        camera::ScreenResolution::new( program_info.dimensions ),
        0.01, 100.0
    );
    let camera_projection = camera.new_projection();
    let mut camera_forward = camera.new_forward();
    let mut camera_view = camera.new_view(camera_forward);
    let camera_up = Vector3::new_up();
    
    // NOTE: Mesh is loaded here! Models generated here!
    let cube_mesh = resources::load_meshes("suzanne.obj").unwrap();
    let floor_meshes = resources::load_meshes( "cube.obj" ).unwrap();

    // NOTE: Textures loaded here!
    let cube_diffuse_texture  = graphics::Texture::new_color_texture(
        color::RGB::new_rgb( 180, 50, 232 ) );
    let cube_specular_texture = graphics::Texture::new_color_texture(
        color::RGB::new_white() * 0.5
    );

    let floor_texture = resources::load_texture(
        "brickwall.jpg", texture::TextureOptions::default()
    ).unwrap();

    // NOTE: Shaders loaded here!
    let cube_shader = resources::load_shader_program("model").unwrap();
    let floor_shader = resources::load_shader_program("fog").unwrap();

    let mut cube_transform = Transform::new_with_position( Vector3::new_back() * 4.0 );
    // cube_transform.scale( &( Vector3::new_one() * 1.2 ) );
    let mut cube_material = Material::new( cube_shader.clone() );

    let cube_trans_loc    = cube_material.get_uniform_location("transform");
    let cube_normal_loc   = cube_material.get_uniform_location("normal_mat");
    let cube_view_loc     = cube_material.get_uniform_location("view");
    let cube_campos_loc   = cube_material.get_uniform_location("camera_position");
    let cube_diffuse_loc  = cube_material.get_uniform_location("diffuse_texture");
    let cube_specular_loc = cube_material.get_uniform_location("specular_texture");

    cube_material[cube_trans_loc].set_matrix4x4( *cube_transform.current_transform_matrix() );
    cube_material[cube_normal_loc].set_matrix3x3( *cube_transform.current_normal_matrix() );
    cube_material[cube_view_loc].set_matrix4x4( camera_view );
    cube_material[cube_campos_loc].set_vector3( *camera.position() );
    cube_material[cube_diffuse_loc].set_sampler2d( ( cube_diffuse_texture.clone(), Sampler::new( 0 ) ) );
    cube_material[cube_specular_loc].set_sampler2d( ( cube_specular_texture.clone(), Sampler::new( 1 ) ) );
    cube_material["projection"].set_matrix4x4( camera_projection );
    cube_material["directional_light.direction"].set_vector3( Vector3::new_up() + Vector3::new_forward() );
    cube_material["directional_light.color"].set_rgb( color::RGB::new_white() * 0.8 );
    cube_material["directional_light.ambient_color"].set_rgb(
        color::RGB::new_rgb( 255, 185, 0 ) * 0.22 );
    cube_material["glossiness"].set_f32( 3.0 );

    let mut floor_material = Material::new( floor_shader.clone() );

    let floor_view_loc = floor_material.get_uniform_location( "view" );
    let floor_diffuse_loc = floor_material.get_uniform_location("diffuse");

    floor_material[floor_view_loc].set_matrix4x4( camera_view );
    floor_material["transform"].set_matrix4x4( Matrix4x4::new_trs_from_vector3(
        &( Vector3::new_down() * 2.0 ),
        &Vector3::new_zero(),
        &Vector3::new( 1000.0, 1.0, 1000.0 )
    ) );
    floor_material["projection"].set_matrix4x4( camera_projection );
    floor_material[floor_diffuse_loc].set_sampler2d( ( floor_texture.clone(), Sampler::new( 0 ) ) );
    floor_material["fog_color"].set_rgb( clear_color );

    let mut _depth_test = graphics::DepthTest::initialize();
    loop {

        // UPDATE -------------------------------------------------------------------------------
        timer.update( sdl_timer.ticks() );
        
        let last_mouse = mouse;
        
        use sdl2::event::Event;
        for event in event_pump.poll_iter() {
            match event { 
                Event::Quit { .. } => { input.quit_game() }
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
        if input.is_quitting() { break; }

        if timer.is_first_frame() {
            mouse.set( 0.0, 0.0 );
            input.set_mouse(mouse);
        }

        input.set_mouse_delta( mouse - last_mouse );

        camera.rotate( &(
            Vector3::new(
                -input.mouse_delta()[1].to_radians(),
                 input.mouse_delta()[0].to_radians(),
                0.0
            ) * timer.delta_time() * mouse_sensitivity
        ) );
        camera.rotation_mut()[0] = camera.rotation()[0]
            .clamp( -75.0f32.to_radians(), 75.0f32.to_radians() );

        camera_forward = camera.new_forward();
        let camera_right = Vector3::cross( &camera_forward, &camera_up);

        let move_direction = {
            let input_direction = input.direction();
            ( camera_right * input_direction[0] ) +
            ( camera_up * input_direction[1] ) +
            ( camera_forward * input_direction[2] )
        }.normal();

        let move_speed = if input.speed_up { fast_camera_speed }
            else { slow_camera_speed };
        let translation = move_direction * move_speed * timer.delta_time();

        camera.translate( &translation );
        camera_view = camera.new_view( camera_forward );

        // RENDER -------------------------------------------------------------------------------
        graphics::clear_screen( gl::DEPTH_BUFFER_BIT );
        {

            floor_material[floor_view_loc].set_matrix4x4( camera_view );
            floor_material.use_material();
            floor_meshes[0].render();

            cube_material[cube_trans_loc].set_matrix4x4( *cube_transform.current_transform_matrix() );
            cube_material[cube_normal_loc].set_matrix3x3( *cube_transform.current_normal_matrix() );
            cube_material[cube_view_loc].set_matrix4x4( camera_view );
            cube_material[cube_campos_loc].set_vector3( *camera.position() );
            cube_material.use_material();
            cube_mesh[0].render();

        } window.gl_swap_window();

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
        icon.width() * ( core::mem::size_of::<u32>() as u32 ),
        sdl2::pixels::PixelFormatEnum::RGBA32
    ).unwrap();

    window.set_icon( &icon_surface );
}

use sdl2::keyboard::Keycode;
fn process_input( input:&mut Input, key_code:Option<Keycode>, is_down:bool ) {
    if !key_code.is_some() { return; }
    match key_code.unwrap() {
        Keycode::W => {
            if is_down { input.forward = true; }
            else { input.forward = false; }
        },
        Keycode::A => {
            if is_down { input.left = true; }
            else { input.left = false; }
        },
        Keycode::S => {
            if is_down { input.back = true; }
            else { input.back = false; }
        },
        Keycode::D => {
            if is_down { input.right = true; }
            else { input.right = false; }
        },
        Keycode::Space => {
            if is_down { input.up = true; }
            else { input.up = false; }
        },
        Keycode::LShift => {
            if is_down { input.speed_up = true; }
            else { input.speed_up = false; }
        },
        Keycode::LCtrl => {
            if is_down { input.down = true; }
            else { input.down = false; }
        },
        Keycode::Escape => {
            if is_down { input.quit_game(); }
        },
        _ => {}
    };
}
