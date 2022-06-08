extern crate fmath;
extern crate sdl2;
extern crate gl;
pub use std::rc::Rc;
use debugging::print_start;

#[allow(unused_imports)]
use gl::types::*;
use graphics::Texture;
#[allow(unused_imports)]
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
#[allow(unused_imports)]
use graphics::{ Camera, camera, Material, texture, UniformBlock,
    light::{ DirectionalLight, PointLight, SpotLight, Lights }
};


#[allow(unused_imports)]
use graphics::Mesh;
use fmath::types::*;
use fmath::functions::lerp;

fn main() {

    resources::initialize();
    let sdl = sdl2::init().unwrap();
    // NOTE: Application Title
    let program_info = resources::load_program_info().unwrap();

    let mut window = {
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(4, 6);
        gl_attr.set_stencil_size( 8 );
        video.window(
            "",
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

    let title = {
        let (major, minor) = debugging::version_str();
        let opengl_version = format!( "OpenGL {}.{}", major, minor );
        let renderer = debugging::gl_str( &gl::RENDERER );
        format!( "{} | {} | {} | FPS: ", opengl_version, program_info.title, renderer )
    };
    window.set_title( &title ).unwrap();

    let mut event_pump = sdl.event_pump().unwrap();
    let sdl_timer = sdl.timer().unwrap();
    let mut timer = Time::new();

    // NOTE: clear color
    // let clear_color = color::RGB::from_hex("#a98bc4").unwrap();
    let clear_color = color::RGB::new_black();
    graphics::set_clear_color( &clear_color );
    graphics::update_viewport( &program_info.dimensions );

    let mut input = Input::new();
    // Camera Speeds
    let slow_camera_speed:f32 = 2.5;
    let fast_camera_speed:f32 = 6.0;
    let mut camera_speed = slow_camera_speed;
    // mouse input buffer
    let mut mouse    = Vector2::new_zero();
    let mouse_sensitivity= 10.0;

    // NOTE: Camera is created here!
    let mut camera = {
        let transform = Transform::new(
            Vector3::new( 0.0, 0.0, 0.0 ),
            Quaternion::new_identity(),
            Vector3::new_one()
        );

        Camera::new(
            transform,
            camera::Projection::perspective_default(),
            camera::ScreenResolution::new( program_info.dimensions ),
            0.01, 25.0
        )
    };

    // NOTE: Mesh is loaded here! Models generated here!
    let suzanne = resources::load_meshes( "suzanne_3.gltf" ).unwrap();
    let floor = resources::load_meshes( "cube.gltf" ).unwrap();
    let floor_transform = Transform::new(
        (Vector3::new_down() * 2.0) + ( Vector3::new_forward() * 25.0 ),
        Quaternion::new_identity(),
        Vector3::new( 50.0, 1.0, 50.0 )
    );

    let floor_texture = resources::load_texture( "brickwall.jpg", None ).unwrap();

    // NOTE: Shaders loaded here!
    let blinn_phong = graphics::null_shader();

    // NOTE: Materials created here!
    let mut suzanne_material = Material::new( blinn_phong.clone() );
    suzanne_material["specular_sampler"].set_texture2d( Texture::new_color_texture(
        color::RGB::new_white() * 0.5
    ) );
    let model_loc = suzanne_material.get_uniform_location("model");
    let normal_loc = suzanne_material.get_uniform_location( "normal_mat" );
    suzanne_material["glossiness"].set_f32( 64.0 );
    suzanne_material["use_vertex_color"].set_bool(true);

    let mut floor_material = Material::new( blinn_phong.clone() );
    let floor_mat = floor_transform.as_matrix();
    floor_material["albedo_sampler"].set_texture2d( floor_texture.clone() );
    floor_material["albedo_sampler_scaler"].set_vector2( Vector2::new_one() * 25.0 );
    floor_material[normal_loc].set_matrix3x3( Matrix3x3::new_normal_matrix( &floor_mat ).unwrap() );
    floor_material[model_loc].set_matrix4x4( floor_mat );

    let mut matrices_block = UniformBlock::new( None, 128 );
    let projection = camera.new_projection().to_le_bytes();
    matrices_block.set_data_slice( &projection, 64 );
    matrices_block.bind_to_buffer_point( 0 );
    blinn_phong.bind_uniform_block_by_name( "Matrices", 0 );

    // NOTE: lights created here!
    let mut light = Lights::new();

    light.directional_light.set_diffuse( color::RGB::new_white() * 0.2 );
    light.directional_light.set_specular( color::RGB::new_white() * 0.2 );

    light.point_lights[0].set_active( true );
    light.point_lights[0].set_position( Vector3::new_right() );
    light.point_lights[0].set_diffuse( color::RGB::new_red() );
    light.point_lights[0].set_specular( color::RGB::new_red() );

    light.point_lights[2].set_active( true );
    light.point_lights[2].set_position( Vector3::new_left() );
    light.point_lights[2].set_diffuse( color::RGB::new_cyan() );
    light.point_lights[2].set_specular( color::RGB::new_cyan() );

    light.spot_lights[0].set_active( true );
    light.spot_lights[0].set_position( Vector3::new( -1.0, 0.0, 1.0 ) );
    light.spot_lights[0].set_diffuse( color::RGB::new_yellow() * 0.8 );
    light.spot_lights[0].set_specular( color::RGB::new_yellow() );
    light.spot_lights[0].set_direction( Vector3::new_forward() );

    let mut spot_light_rotation = AngleAxis::new( 0.0, Vector3::new_up() );

    let mut lights_block = UniformBlock::new( Some( light.as_bytes() ), light.size() );
    lights_block.bind_to_buffer_point( 1 );
    blinn_phong.bind_uniform_block_by_name( "Lights", 1 );

    let mut data_block = UniformBlock::new( None, 40 );
    let fog_color = clear_color.as_vector4();
    data_block.set_data_slice( &fog_color.to_le_bytes(), 16 );
    data_block.set_data_slice( &camera.near_clip().to_le_bytes(), 32 );
    data_block.set_data_slice( &camera.far_clip().to_le_bytes(), 36 );
    data_block.bind_to_buffer_point( 2 );
    blinn_phong.bind_uniform_block_by_name( "Data", 2 );

    let mut cam_yaw = AngleAxis::new( -180.0f32.to_radians(), Vector3::new_up() );
    let mut cam_pitch = AngleAxis::new( 0.0, Vector3::new_right() );
    let mut _depth_test = graphics::DepthTest::initialize();
    loop {

        // UPDATE -------------------------------------------------------------------------------
        timer.update( sdl_timer.ticks() );
        window.set_title( &format!( "{}{}", title, timer.fps() as u32 ) ).unwrap();

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

        input.set_mouse_delta( mouse - last_mouse );

        cam_yaw   += -input.mouse_delta()[0].to_radians() * timer.unscaled_delta_time() * mouse_sensitivity;
        cam_pitch += input.mouse_delta()[1].to_radians()  * timer.unscaled_delta_time() * mouse_sensitivity;
        cam_pitch.clamp_to( -1.309, 1.309 );
        
        camera.transform.set_rotation( Quaternion::from_angle_axis( cam_yaw ) );
        cam_pitch.set_axis( camera.transform.new_right() );
        camera.transform.rotate( Quaternion::from_angle_axis( cam_pitch ) );

        let camera_basis = camera.transform.new_basis();

        let move_forward = { Quaternion::from_angle_axis( cam_yaw ) * Vector3::new_forward() };

        let input_direction = input.direction();
        let mut move_direction = {
            ( camera_basis.right * -input_direction[0] ) +
            ( move_forward       *  input_direction[2] )
        }.normal();

        move_direction += Vector3::new_up() * input_direction[1];

        let camera_speed_target = if input.speed_up { fast_camera_speed }
            else { slow_camera_speed };
        camera_speed = lerp( camera_speed, camera_speed_target, timer.unscaled_delta_time() * 10.0 );
        let translation = move_direction * camera_speed * timer.unscaled_delta_time();

        camera.transform.translate( translation );
        let view = camera.new_view( camera_basis.forward ).to_le_bytes();

        light.spot_lights[0].set_direction(
            Quaternion::from_angle_axis( spot_light_rotation ) *
            Vector3::new_forward()
        );
        spot_light_rotation += ( timer.delta_time() * 60.0 ).to_radians();

        // update uniform blocks
        lights_block.use_block();
        lights_block.set_data_slice(
            &light.spot_lights[0].direction().to_le_bytes(),
            light.spot_light_offset(0) + light.spot_lights[0].direction_offset()
        );

        matrices_block.use_block();
        matrices_block.set_data_slice( &view, 0 );

        let camera_position = camera.transform.position().to_le_bytes();
        data_block.use_block();
        data_block.set_data_slice( &camera_position, 0 );

        // RENDER -------------------------------------------------------------------------------
        graphics::clear_screen( gl::DEPTH_BUFFER_BIT );
        {

            suzanne_material.use_shader();
            let mut rot_aa = AngleAxis::new( 0.0, Vector3::new_up() );
            let mut angle_modifier = 0.0;
            let mut dist = 3.0;
            for i in 0..80 {
                
                let rot = Quaternion::from_angle_axis( rot_aa ).normal();
                let transform = Transform::new(
                    rot * ( Vector3::new_back() * dist ),
                    rot,
                    Vector3::new_one()
                );

                match i % 4 {
                    0 => {
                        rot_aa.set_angle( (0f32 + angle_modifier).to_radians() );
                        if i > 0 {
                            dist += 2.0;
                            angle_modifier += 15.0;
                        }
                    },
                    1 => rot_aa.set_angle( (90f32 + angle_modifier).to_radians() ),
                    2 => rot_aa.set_angle( (180f32 + angle_modifier).to_radians() ),
                    3 => rot_aa.set_angle( (270f32 + angle_modifier).to_radians() ),
                    _ => {}
                };
                
                let model_mat = transform.as_matrix();

                suzanne_material[normal_loc].set_matrix3x3( Matrix3x3::new_normal_matrix( &model_mat ).unwrap() );
                suzanne_material[model_loc].set_matrix4x4( model_mat );
                suzanne_material.send_all_uniforms_to_gl();
                for mesh in suzanne.iter() { mesh.render(); }
            }

            floor_material.send_all_uniforms_to_gl();
            floor[0].render();

        } window.gl_swap_window();

    }

}

pub struct ProgramInfo {
    pub title: String,
    pub dimensions: Vector2,
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
