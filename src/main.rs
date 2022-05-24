extern crate fmath;
extern crate sdl2;
extern crate gl;
pub use std::rc::Rc;

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

fn main() {

    resources::initialize();
    let sdl = sdl2::init().unwrap();
    // NOTE: Application Title
    let program_info = resources::load_program_info().unwrap();

    let mut window = {
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(3, 3);
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

    let gl_ctx = window.gl_create_context().unwrap();
    graphics::load_glfn( window.subsystem() );

    let mut event_pump = sdl.event_pump().unwrap();
    let sdl_timer = sdl.timer().unwrap();
    let mut timer = Time::new();

    // NOTE: clear color
    // let clear_color = color::RGB::from_hex("#a98bc4").unwrap();
    let clear_color = color::RGB::new_black();
    graphics::clear_color( &clear_color );
    graphics::set_viewport( &program_info.dimensions );

    let mut input = Input::new();
    // NOTE: Camera Speeds
    let slow_camera_speed:f32 = 1.5;
    let fast_camera_speed:f32 = 3.0;

    let mut camera = camera::Camera::new( camera::ProjectionMode::Perspective )
        .set_aspect_ratio( program_info.aspect_ratio() )
        .set_fov( d2r( 65.0 ) )
        .set_clipping_fields( 0.01, 100.0 )
        .set_rotation( Vector3::new( 0.0, d2r( -90.0 ), 0.0 ) )
    .build();

    // NOTE: Mesh is loaded here!
    let cube_mesh  = resources::load_meshes( "cube.obj" ).unwrap();
    let suzanne_mesh = resources::load_meshes( "suzanne.obj" ).unwrap();

    let model_shader = resources::load_shader_program( "model" ).unwrap();
    let fog_shader = resources::load_shader_program( "fog" ).unwrap();
    let flat_shader = resources::load_shader_program( "flat" ).unwrap();

    let model_diffuse_texture =
        graphics::Texture::new_color_texture( color::RGB::from_hex( "#c77be8" ).unwrap() );
    let model_specular_texture =
        graphics::Texture::new_color_texture( color::RGB::new_gray() );
    let floor_texture = resources::load_texture(
            "brickwall.jpg",
            graphics::texture::TextureOptions::default()
        ).unwrap();

    let mut floor_material   = Material::new( "floor_material", fog_shader.clone() );
    let mut model_material   = Material::new( "model_material", model_shader.clone() );
    let mut outline_material = Material::new( "outline_material", flat_shader.clone() );

    let mut model_transform = Transform::new_with_position( Vector3::new_back() * 5.0 );

    let floor_model  = Model::new( cube_mesh.clone() );
    let model_model  = Model::new( suzanne_mesh.clone() );
    
    // set up floor material ------------------------------------------------------------------

    let view_floor    = floor_material.get_uniform_location("view").unwrap();
    floor_material["transform"].set_matrix4(
        *Transform::new(
            Vector3::new_down() * 2.0,
            Vector3::new_zero(),
            Vector3::new( 1000.0, 1.0, 1000.0 )
        ).transform_matrix()
    );
    floor_material["projection"].set_matrix4( *camera.projection() );
    floor_material["diffuse"].set_texture(
        floor_texture.clone(),
        graphics::Sampler::new( 0 )
    );
    floor_material["fog_color"].set_rgb( clear_color );

    // set up model material ------------------------------------------------------------------

    let mt_loc      = model_material.get_uniform_location("transform").unwrap();
    let mview_loc   = model_material.get_uniform_location("view").unwrap();
    let mnorm_loc   = model_material.get_uniform_location( "normal_mat" ).unwrap();
    let mcampos_loc = model_material.get_uniform_location( "camera_position" ).unwrap();

    model_material["projection"].set_matrix4( *camera.projection() );
    model_material["directional_light.direction"].set_vector3( Vector3::new_one() );
    model_material["directional_light.color"].set_rgb( color::RGB::new_white() );
    model_material["directional_light.ambient_color"].set_rgb( color::RGB::new_red() * 0.15 );
    model_material["glossiness"].set_f32( 32.0 );
    model_material["diffuse_texture"].set_texture(
        model_diffuse_texture.clone(),
        graphics::Sampler::new( 0 )
    );
    model_material["specular_texture"].set_texture(
        model_specular_texture.clone(),
        graphics::Sampler::new( 1 )
    );

    // set up outline material ------------------------------------------------------------------
    let outline_size = 0.03;
    let transform_outline_loc = outline_material.get_uniform_location("transform").unwrap();
    let view_outline_loc      = outline_material.get_uniform_location("view").unwrap();
    
    outline_material["projection"].set_matrix4( *camera.projection() );
    outline_material[transform_outline_loc].set_matrix4( *model_transform.transform_matrix() );
    outline_material["color"].set_rgb( color::RGB::new_white() );

    let mut mouse = Vector2::new_zero();
    let mouse_sensitivity = 10.0;
    let mut _depth_test = graphics::DepthTest::initialize();
    let mut _stencil_test = graphics::StencilTest::initialize();
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

        camera.transform.rotate( &(
            Vector3::new(
                d2r( -input.mouse_delta()[1] ),
                d2r(  input.mouse_delta()[0] ),
                0.0
            ) * timer.delta_time() * mouse_sensitivity
        ) );
        camera.transform.rotation_mut()[0] = camera.transform.rotation()[0].clamp( d2r( -75.0 ), d2r( 75.0 ) );
        camera.transform.update_basis_vectors();

        let move_direction = {
            let input_direction = input.direction();
            ( *camera.transform.right()   * input_direction[0] ) +
            ( *camera.transform.up()      * input_direction[1] ) +
            ( *camera.transform.forward() * input_direction[2] )
        }.normal();

        let move_speed = if input.speed_up { fast_camera_speed }
            else { slow_camera_speed };
        let translation = move_direction * move_speed * timer.delta_time();
        
        camera.transform.translate( &translation );
        let camera_view = camera.view();

        model_transform.update_transform_matrix();
        model_transform.update_normal_matrix();

        // RENDER -------------------------------------------------------------------------------
        use graphics::{ StencilAction, TestKind };
        _depth_test.enable();
        _stencil_test.op(
            StencilAction::Keep,
            StencilAction::Keep,
            StencilAction::Replace
        );
        graphics::clear_screen( gl::DEPTH_BUFFER_BIT | gl::STENCIL_BUFFER_BIT );
        {

            // DRAW FLOOR -------------------------------------------

            // don't update stencil while drawing floor
            _stencil_test.set_stencil_mask( 0x00 );

            floor_material[view_floor].set_matrix4( camera_view );
            floor_model.render( &floor_material );

            // DRAW CUBE --------------------------------------------

            // all fragments pass test
            _stencil_test.func( TestKind::Always, 1, 0xFF );
            // enable writing to stencil buffer
            _stencil_test.set_stencil_mask( 0xFF );

            model_material[mt_loc].set_matrix4( *model_transform.transform_matrix() );
            model_material[mnorm_loc].set_matrix3( *model_transform.normal_matrix() );
            model_material[mview_loc].set_matrix4( camera_view );
            model_material[mcampos_loc].set_vector3( *camera.transform.position() );
            model_model.render( &model_material );

            // DRAW CUBE OUTLINE -----------------------------------

            _stencil_test.func( TestKind::NotEqual, 1, 0xFF );
            // disable writing to the stencil buffer
            _stencil_test.set_stencil_mask( 0x00 );
            // disable depth testing
            _depth_test.disable();

            let outline_transform_mat = Matrix4x4::new_trs(
                &model_transform.position().as_array(),
                &model_transform.rotation().as_array(),
                &( *model_transform.size() * ( 1.0 + outline_size ) ).as_array(),
            );

            outline_material[view_outline_loc].set_matrix4( camera_view );
            outline_material[transform_outline_loc].set_matrix4( outline_transform_mat );
            model_model.render( &outline_material );

            _stencil_test.set_stencil_mask( 0xFF );
            _stencil_test.func( TestKind::Always, 1, 0xFF );

        } window.gl_swap_window();

    }

    // clean up
    {
        let materials = vec![
            model_material,
            floor_material,
            outline_material
        ];
        drop( materials );
        
        let textures = vec![
            floor_texture,
        ];
        unsafe { graphics::texture::delete_textures( textures ) };

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
