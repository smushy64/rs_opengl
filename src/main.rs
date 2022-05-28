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
        Vector3::new( 0.0, 0.6, 0.0 ),
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
    let suzanne_meshes = resources::load_meshes( "suzanne.obj" ).unwrap();
    let floor_meshes = resources::load_meshes( "cube.obj" ).unwrap();
    let window_mesh = graphics::mesh::generate_plane();
    let window_transforms = [ Transform::new(
        Vector3::new( 0.4, 0.6, -0.8 ),
            Vector3::new_zero(),
            Vector3::new_one()
        ), Transform::new(
            Vector3::new( 0.0, 0.6, -1.5 ),
            Vector3::new_zero(),
            Vector3::new_one()
        ),
    ];

    // NOTE: Textures loaded here!
    let suzanne_diffuse_texture  = graphics::Texture::new_color_texture(
        color::RGB::new_rgb( 245, 200, 0 ) );
    let suzanne_specular_texture = graphics::Texture::new_color_texture(
        color::RGB::new_white() * 0.5
    );

    let mut window_texture_options = texture::TextureOptions::default();
    window_texture_options.set_wrapping( texture::TextureWrapping::ClampToEdge );

    let window_texture = resources::load_texture(
        "window.png", window_texture_options
    ).unwrap();

    let floor_texture = resources::load_texture(
        "brickwall.jpg", texture::TextureOptions::default()
    ).unwrap();

    // NOTE: Shaders loaded here!
    let suzanne_shader = resources::load_shader_program("model").unwrap();
    let floor_shader = resources::load_shader_program("fog").unwrap();
    let transparency_shader = resources::load_shader_program("transparency").unwrap();

    let mut suzanne_transform = Transform::new_with_position( Vector3::new_back() * 4.0 );
    let mut suzanne_material = Material::new( suzanne_shader.clone() );

    let suzanne_transform_loc = suzanne_material.get_uniform_location("transform");
    let suzanne_normal_loc    = suzanne_material.get_uniform_location("normal_mat");
    let suzanne_view_loc      = suzanne_material.get_uniform_location("view");
    let suzanne_campos_loc    = suzanne_material.get_uniform_location("camera_position");
    let suzanne_diffuse_loc   = suzanne_material.get_uniform_location("diffuse_texture");
    let suzanne_specular_loc  = suzanne_material.get_uniform_location("specular_texture");

    suzanne_material[suzanne_transform_loc].set_matrix4x4( *suzanne_transform.current_transform_matrix() );
    suzanne_material[suzanne_normal_loc].set_matrix3x3( *suzanne_transform.current_normal_matrix() );
    suzanne_material[suzanne_view_loc].set_matrix4x4( camera_view );
    suzanne_material[suzanne_campos_loc].set_vector3( *camera.position() );
    suzanne_material[suzanne_diffuse_loc].set_sampler2d( ( suzanne_diffuse_texture.clone(), Sampler::new( 0 ) ) );
    suzanne_material[suzanne_specular_loc].set_sampler2d( ( suzanne_specular_texture.clone(), Sampler::new( 1 ) ) );
    suzanne_material["projection"].set_matrix4x4( camera_projection );
    suzanne_material["directional_light.direction"].set_vector3( Vector3::new_up() + Vector3::new_forward() );
    suzanne_material["directional_light.color"].set_rgb( color::RGB::new_white() * 0.8 );
    suzanne_material["directional_light.ambient_color"].set_rgb(
        color::RGB::new_rgb( 255, 185, 0 ) * 0.12 );
    suzanne_material["glossiness"].set_f32( 3.0 );

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

    let mut window_material = Material::new( transparency_shader.clone() );

    let window_transform_loc = window_material.get_uniform_location("transform");
    let window_view_loc = window_material.get_uniform_location("view");
    let window_texture_loc = window_material.get_uniform_location("diffuse");

    window_material["projection"].set_matrix4x4( camera_projection );
    window_material[window_texture_loc].set_sampler2d( ( window_texture.clone(), Sampler::new( 0 ) ) );

    use graphics::BlendFactor;
    let mut _blend = Box::new( graphics::Blend::initialize() );
    _blend.set_blend_factor( BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha );
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

        suzanne_transform.rotate( &Vector3::new( 0.0, timer.delta_time() * 2.0, 0.0 ) );
        *suzanne_transform.position_mut() = Vector3::new(
            suzanne_transform.position()[0],
            (timer.time().sin() + 1.2) * 0.5,
            suzanne_transform.position()[2]
        );

        camera.translate( &translation );
        camera_view = camera.new_view( camera_forward );

        // RENDER -------------------------------------------------------------------------------
        graphics::clear_screen( gl::DEPTH_BUFFER_BIT );
        {

            floor_material[floor_view_loc].set_matrix4x4( camera_view );
            floor_material.use_material();
            floor_meshes[0].render();

            suzanne_material[suzanne_transform_loc].set_matrix4x4( *suzanne_transform.current_transform_matrix() );
            suzanne_material[suzanne_normal_loc].set_matrix3x3( *suzanne_transform.current_normal_matrix() );
            suzanne_material[suzanne_view_loc].set_matrix4x4( camera_view );
            suzanne_material[suzanne_campos_loc].set_vector3( *camera.position() );
            suzanne_material.use_material();
            suzanne_meshes[0].render();

            window_material.use_material();
            window_material[window_view_loc].set_matrix4x4( camera_view );
            
            let mut sorted = window_transforms.clone();
            sorted.sort_by(
                | a, b | {
                    let dist_a =
                        ( *camera.position() - *a.position() ).sqr_magnitude();
                    let dist_b =
                        ( *camera.position() - *b.position() ).sqr_magnitude();
                    dist_a.partial_cmp( &dist_b ).unwrap()
                }
            );

            for transform in sorted.iter().rev() {
                window_material[window_transform_loc].set_matrix4x4( *transform.transform_matrix() );
                window_material.send_uniforms_to_gl();
                window_mesh.render();
            }

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
