mod material;
mod vertex;
mod model;

pub mod shader;
pub use shader::{ Shader, ShaderProgram, Error as ShaderError };

pub mod mesh;
pub use mesh::Mesh;

pub mod texture;
pub use texture::{ Texture, Sampler };

pub mod uniform;
pub use uniform::{ MaterialUniform, UniformValue };

pub use material::Material;
pub use vertex::Vertex;
pub use model::Model;

use gl::types::*;

pub fn load_glfn( subsys:&sdl2::VideoSubsystem ) {
    gl::load_with(
        |symbol|
            subsys.gl_get_proc_address(&symbol) as *const GLvoid
    );
}

pub fn clear_color( color:&fmath::types::color::RGB ) {
    let rgb = color.as_tuple_rgb_f32();
    unsafe {
        gl::ClearColor( rgb.0, rgb.1, rgb.2, 1.0 );
    }
}

pub fn clear_screen() { unsafe { gl::Clear( gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT ); } }

pub fn set_viewport( dimensions:&fmath::types::Vector2 ) {
    unsafe {
        gl::Viewport(
            0 as GLint, 0 as GLint,
            dimensions[0] as GLsizei, dimensions[1] as GLsizei
        );
    }
}
