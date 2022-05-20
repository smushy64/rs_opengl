extern crate sdl2;
extern crate fmath;
extern crate gl;
extern crate wavefront_obj_importer;
extern crate rand;

use std::rc::Rc;

use rand::{thread_rng, Rng};

pub mod resources;
pub mod shader;
pub mod c_string;
pub mod opengl_fn;
pub mod input;
pub mod mesh;
pub mod light;
pub mod texture;
use texture::{ Texture, TexCoord };

use input::Input;
pub mod transform;
use transform::Transform;

#[allow(unused_imports)]
use gl::types::*;

use fmath::types::*;
use fmath::functions::angles::degrees_to_radians as d2r;

fn main() {

    resources::initialize();
    let mut rng = thread_rng();

    let sdl = sdl2::init().unwrap();

    // NOTE: Application Title
    let title = "OpenGL | Multiple Light Sources Demo";
    let dimensions  = Vector2::new( 1280.0, 720.0 );

    let mut window = {
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile( sdl2::video::GLProfile::Core );
        gl_attr.set_context_version(3, 3);
        video.window( title, dimensions[0] as u32, dimensions[1] as u32 )
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
    let timer     = sdl.timer().unwrap();

    // NOTE: clear color
    // let clear_color = color::RGB::from_hex("#776094").unwrap();
    // let clear_color = color::RGB::from_hex("#b9d9eb").unwrap();
    let clear_color = color::RGB::new_black();
    opengl_fn::set_clear_color( &clear_color );
    opengl_fn::set_viewport( &dimensions );

    let cube_count = 10;
    let mut cube_transforms:Vec<Transform> = Vec::with_capacity( cube_count );
    let mut counter = 0;

    let xy_range = 5.0;
    let z_range  = 10.0;
    let rot_range = 180.0;

    while counter < cube_count {
        cube_transforms.push( Transform::new() );

        cube_transforms[counter].set_position(
            Vector3::new(
                rng.gen_range( -xy_range..xy_range ),
                rng.gen_range( -xy_range..xy_range ),
                rng.gen_range( -z_range..-1.0 )
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

    let aspect_ratio:f32 = dimensions[0] / dimensions[1];
    let perspective_projection = opengl_fn::persp(
        d2r(80.0),
        aspect_ratio,
        0.01, 100.0
    );

    let directional_light = light::DirectionalLight{
        direction: Vector3::new_down(),
        diffuse:   color::RGB::from_float_array_rgb([ 0.4, 0.4, 0.41 ]),
        specular:  color::RGB::from_float_array_rgb([ 0.4, 0.4, 0.41 ]),
        ambient:   color::RGB::new_black(),
    };

    let default_shader = resources::load_shader_program( "shaders/default" ).unwrap();
    // NOTE: using default shader!
    default_shader.use_program();
    default_shader.set_matrix4_by_name( "projection", &perspective_projection );

    let container_diffuse = resources::load_texture( "container2.png" ).unwrap()
        .set_linear_filtering()
        .set_texcoord( TexCoord::ID(0) )
        .build();

    let container_specular = resources::load_texture( "container2_specular.png" ).unwrap()
        .set_linear_filtering()
        .set_texcoord( TexCoord::ID(1) )
        .build();

    container_diffuse.use_texture();
    container_specular.use_texture();

    default_shader.set_sampler_by_name(
        "material.diffuse_sampler",
        container_diffuse.texcoord()
    );
    default_shader.set_sampler_by_name(
        "material.specular_sampler",
        container_specular.texcoord()
    );
    default_shader.set_f32_by_name( "material.glossiness", &32.0 );

    default_shader.set_vector3_by_name(
        "directional_light.direction",
        &directional_light.direction
    );
    default_shader.set_rgb_by_name( "directional_light.ambient",  &directional_light.ambient  );
    default_shader.set_rgb_by_name( "directional_light.diffuse",  &directional_light.diffuse  );
    default_shader.set_rgb_by_name( "directional_light.specular", &directional_light.specular );

    let cube_mesh = Rc::new( mesh::generate_cube() );

    let ambient_hsv_value = 0.1;
    let ( mut point_lights, mut light_hsv_list ) = {

        let count = 4;
        let mut lights:Vec<light::PointLight> = Vec::with_capacity( count );
        let mut hsv:Vec<color::HSV> = Vec::with_capacity( count );
        let mut i = 0;
        while i < count {

            let mut light_hsv = color::HSV::new(
                rng.gen_range( 0.0..360.0 ),
                1.0, 1.0
            );

            hsv.push( light_hsv.clone() );

            let light_col = light_hsv.as_rgb();
            light_hsv.set_value( ambient_hsv_value );
            let ambient = light_hsv.as_rgb();

            lights.push(
                light::PointLight {

                    position: Vector3::new(
                        rng.gen_range( -xy_range..xy_range ),
                        rng.gen_range( -xy_range..xy_range ),
                        rng.gen_range( -z_range..-1.0 )
                    ),

                    diffuse:  light_col.clone(),
                    specular: light_col.clone(),
                    ambient,

                    constant:  1.0,
                    linear:    0.14,
                    quadratic: 0.07,

                    mesh: cube_mesh.clone()

                }
            );

            i += 1;

        }

        ( lights, hsv )

    };

    for ( idx, light ) in point_lights.iter().enumerate() { 

        default_shader.set_vector3_by_name( &format!("point_lights[{}].position", idx ),
            &light.position
        );

        default_shader.set_f32_by_name( &format!("point_lights[{}].constant", idx ),
            &light.constant
        );

        default_shader.set_f32_by_name( &format!("point_lights[{}].linear", idx ),
            &light.linear
        );

        default_shader.set_f32_by_name( &format!("point_lights[{}].quadratic", idx ),
            &light.quadratic
        );

    }

    let mut spot_light = light::SpotLight {
        position:  camera_position.clone(),
        direction: camera_front.clone(),

        inner_cutoff:    d2r(25.0).cos(),
        outer_cutoff:    d2r(28.0).cos(),

        constant:  1.0,
        linear:    0.14,
        quadratic: 0.07,

        diffuse:  color::RGB::new_white(),
        specular: color::RGB::new_white(),
        ambient:  color::RGB::new_white() * 0.1,

    };

    default_shader.set_vector3_by_name( "spot_lights[0].position", &spot_light.position );
    default_shader.set_vector3_by_name( "spot_lights[0].direction", &spot_light.direction );

    default_shader.set_f32_by_name( "spot_lights[0].inner_cutoff", &spot_light.inner_cutoff );
    default_shader.set_f32_by_name( "spot_lights[0].outer_cutoff", &spot_light.outer_cutoff );

    default_shader.set_f32_by_name( "spot_lights[0].constant", &spot_light.constant );
    default_shader.set_f32_by_name( "spot_lights[0].linear", &spot_light.linear );
    default_shader.set_f32_by_name( "spot_lights[0].quadratic", &spot_light.quadratic );

    default_shader.set_rgb_by_name( "spot_lights[0].diffuse", &color::RGB::new_black() );
    default_shader.set_rgb_by_name( "spot_lights[0].specular", &color::RGB::new_black() );
    default_shader.set_rgb_by_name( "spot_lights[0].ambient", &color::RGB::new_black() );

    let flat_shader = resources::load_shader_program( "shaders/flat" ).unwrap();

    flat_shader.use_program();

    flat_shader.set_matrix4_by_name( "projection", &perspective_projection );

    unsafe { gl::Enable( gl::DEPTH_TEST ); }
    let mut running:bool = true;
    while running {

        use sdl2::event::Event;

        let elapsed = (timer.ticks() as f32) / 1000.0;
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

        let camera_transform = opengl_fn::new_look_at_mat(
            &camera_position,
            &( camera_position + camera_front ),
            &camera_up
        );

        opengl_fn::clear_screen();

        default_shader.use_program();

        default_shader.set_matrix4_by_name("camera_transform", &camera_transform);
        default_shader.set_vector3_by_name("camera_position", &camera_position);
        
        spot_light.position  = camera_position.clone();
        spot_light.direction = camera_front.clone();

        default_shader.set_vector3_by_name( "spot_lights[0].position", &spot_light.position );
        default_shader.set_vector3_by_name( "spot_lights[0].direction", &spot_light.direction );

        if input.flashlight_updated {
            if input.flashlight {
                default_shader.set_rgb_by_name( "spot_lights[0].diffuse", &spot_light.diffuse );
                default_shader.set_rgb_by_name( "spot_lights[0].specular", &spot_light.specular );
                default_shader.set_rgb_by_name( "spot_lights[0].ambient", &spot_light.ambient );
            } else {
                default_shader.set_rgb_by_name( "spot_lights[0].diffuse", &color::RGB::new_black() );
                default_shader.set_rgb_by_name( "spot_lights[0].specular", &color::RGB::new_black() );
                default_shader.set_rgb_by_name( "spot_lights[0].ambient", &color::RGB::new_black() );
            }
        }

        for ( idx, light ) in point_lights.iter().enumerate() {

            let name = format!("point_lights[{}].", idx);

            default_shader.set_rgb_by_name( &format!("{}ambient", name ),
                &light.ambient
            );

            default_shader.set_rgb_by_name( &format!("{}diffuse", name ),
                &light.diffuse
            );

            default_shader.set_rgb_by_name( &format!("{}specular", name ),
                &light.specular
            );
        }

        for cube_transform in cube_transforms.iter_mut() {

            cube_transform.set_rotation(
                *cube_transform.rotation() +
                (Vector3::new( 0.2, 0.1, 0.3 ) * delta_time)
            );

            render_mesh( &cube_mesh, &default_shader, &cube_transform );
        }

        flat_shader.use_program();

        flat_shader.set_matrix4_by_name( "camera_transform", &camera_transform );
        for ( idx, light ) in point_lights.iter_mut().enumerate() {

            let mut current_hsv = light_hsv_list[idx].clone();

            current_hsv.set_hue( current_hsv.hue() + delta_time * 50.0 );
            light_hsv_list[idx] = current_hsv.clone();

            let rgb = current_hsv.as_rgb();

            light.diffuse  = rgb;
            light.specular = rgb;

            current_hsv.set_value( ambient_hsv_value );

            light.ambient = current_hsv.as_rgb();

            render_point_light( light, &flat_shader );

        }

        window.gl_swap_window();

        input.flashlight_updated = false;

    }

    let textures:Vec<Texture> = Vec::from([
        container_diffuse, container_specular
    ]);

    texture::delete_textures( textures );

    drop( gl_ctx );
    drop( sdl );

}

fn render_point_light( light:&light::PointLight, shader:&shader::ShaderProgram ) {
    shader.set_rgb_by_name( "color", &light.diffuse );
    render_mesh( &light.mesh, &shader, &light.transform() );
}

fn render_mesh( mesh:&mesh::Mesh, shader:&shader::ShaderProgram, transform:&Transform ) {

    shader.set_matrix4_by_name( "transform", transform.mat() );
    mesh.render();

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
                Keycode::F => {
                    if is_down {
                        input.toggle_flashlight();
                    }
                }
                _ => {}
            }
        },
        None => {},
    }
}
