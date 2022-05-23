use gl::types::*;
use fmath::types::*;
use crate::{
    c_string,
    opengl_fn,
    texture::TexCoord,
};

#[derive(Clone)]
pub struct ShaderProgram {
    handle: GLuint
}

impl ShaderProgram {

    pub fn get_uniform_location( &self, name:&str ) -> GLint {
        unsafe {
            let c_name = c_string::c_string_from_str(name).unwrap();
            let location =
                gl::GetUniformLocation( self.handle() , c_name.as_ptr() as *const GLchar );
            return location;
        }
    }

    pub fn set_sampler( &self, loc:GLint, value:&TexCoord ) {
        self.set_i32( loc, value.get_id() );
    }

    pub fn set_sampler_by_name( &self, name:&str, value:&TexCoord ) {
        self.set_sampler( self.get_uniform_location( name ), value );
    }

    pub fn set_u32( &self, loc:GLint, value:&u32 ) {
        unsafe {
            gl::Uniform1ui( loc, *value );
        }
    }

    pub fn set_u32_by_name( &self, name:&str, value:&u32 ) {
        self.set_u32( self.get_uniform_location( name ), value )
    }

    pub fn set_i32( &self, loc:GLint, value:&i32 ) {
        unsafe {
            gl::Uniform1i( loc, *value );
        }
    }

    pub fn set_i32_by_name( &self, name:&str, value:&i32 ) {
        self.set_i32( self.get_uniform_location( name ), value )
    }

    pub fn set_f32( &self, loc:GLint, value:&f32 ) {
        unsafe {
            gl::Uniform1f( loc, *value );
        }
    }

    pub fn set_f32_by_name( &self, name:&str, value:&f32 ) {
        self.set_f32( self.get_uniform_location(name), value );
    }

    pub fn set_matrix4( &self, loc:GLint, value:&Matrix4x4 ) {
        unsafe {
            gl::UniformMatrix4fv(
                loc, 1, 
                gl::FALSE, value.as_array().as_ptr()
            );
        }
    }

    pub fn set_matrix4_by_name( &self, name:&str, value:&Matrix4x4 ) {
        self.set_matrix4( self.get_uniform_location(name), value );
    }

    pub fn set_matrix3( &self, loc:GLint, value:&Matrix3x3 ) {
        unsafe {
            gl::UniformMatrix3fv(
                loc, 1, 
                gl::FALSE, value.as_array().as_ptr()
            );
        }
    }

    pub fn set_matrix3_by_name( &self, name:&str, value:&Matrix3x3 ) {
        self.set_matrix3( self.get_uniform_location(name), value );
    }

    pub fn set_vector3( &self, loc:GLint, value:&Vector3 ) {
        unsafe {
            gl::Uniform3fv( loc, 1, value.as_array().as_ptr() );
        }
    }

    pub fn set_vector3_by_name( &self, name:&str, value:&Vector3 ) {
        self.set_vector3( self.get_uniform_location(name), value );
    }

    pub fn set_vector4( &self, loc:GLint, value:&Vector4 ) {
        unsafe {
            gl::Uniform4fv( loc, 1, value.as_array().as_ptr() );
        }
    }

    pub fn set_vector4_by_name( &self, name:&str, value:&Vector4 ) {
        self.set_vector4( self.get_uniform_location(name), value );
    }

    pub fn set_rgb( &self, loc:GLint, value:&color::RGB ) {
        unsafe {
            gl::Uniform3fv( loc, 1, value.as_array_rgb_f32().as_ptr() );
        }
    }

    pub fn set_rgb_by_name( &self, name:&str, value:&color::RGB ) {
        self.set_rgb( self.get_uniform_location(name), value );
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram( self.handle() ); }
    }

    pub fn handle(&self) -> GLuint {
        self.handle
    }

    pub fn from_shaders( shaders: &[Shader] ) -> Result<Self, Error> {
        unsafe {
            let handle:GLuint = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader( handle, shader.handle() );
            }

            gl::LinkProgram( handle );

            let mut link_status:GLint = 1;
            gl::GetProgramiv( handle, gl::LINK_STATUS, &mut link_status );

            if FAILED == link_status {
                return Err(
                    Error::Linking( opengl_fn::gl_error_linking( handle ) )
                )
            }

            for shader in shaders {
                gl::DetachShader( handle, shader.handle() );
            }

            return Ok( Self { handle } );

        }
    }

}

impl Drop for ShaderProgram {
    fn drop( &mut self ) {
        unsafe { gl::DeleteProgram( self.handle() ) }
    }
}

pub struct Shader {
    handle: GLuint,
}

impl Shader {

    pub fn handle(&self) -> GLuint {
        self.handle
    }

    pub fn vert_from_source( src: &c_string::CStr ) -> Result<Self, Error> {
        Self::from_source(src, ShaderKind::Vertex)
    }

    pub fn frag_from_source( src: &c_string::CStr ) -> Result<Self, Error> {
        Self::from_source(src, ShaderKind::Fragment)
    }

    fn from_source( src: &c_string::CStr, kind:ShaderKind ) -> Result<Self, Error> {

        let handle:GLuint = Self::create_shader( kind );
        unsafe {
            gl::ShaderSource(
                handle, 1,
                &src.as_ptr(), core::ptr::null()
            );
            gl::CompileShader( handle );
        }

        let mut compile_status:GLint = 1;
        unsafe {
            gl::GetShaderiv( handle, gl::COMPILE_STATUS, &mut compile_status );
        }

        if FAILED == compile_status {
            return Err(
                Error::Compilation( opengl_fn::gl_error_compilation( handle ))
            );
        }

        return Ok( Self{ handle } );

    }

    fn create_shader( kind:ShaderKind ) -> GLuint {
        match kind {
            ShaderKind::Vertex   => unsafe { gl::CreateShader( gl::VERTEX_SHADER ) },
            ShaderKind::Fragment => unsafe { gl::CreateShader( gl::FRAGMENT_SHADER ) },
        }
    }

}

impl Drop for Shader {

    fn drop( &mut self ) {
        unsafe {
            gl::DeleteShader( self.handle() );
        }
    }

}

enum ShaderKind {
    Vertex, Fragment
}

const FAILED:GLint = 0;

#[derive(Debug)]
pub enum Error {
    Linking(String),
    Compilation(String),
}
