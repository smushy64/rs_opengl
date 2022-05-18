use gl::types::*;
use fmath::types::*;
use super::c_string;
use super::opengl_fn;

#[derive(Clone)]
pub struct ShaderProgram {
    id: GLuint
}

impl ShaderProgram {

    pub fn get_uniform_location( &self, name:&str ) -> GLint {
        unsafe {
            let c_name = c_string::c_string_from_str(name).unwrap();
            let location =
                gl::GetUniformLocation( self.id() , c_name.as_ptr() as *const GLchar );
            return location;
        }
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
            gl::Uniform3fv( loc, 1, value.as_float_rgb_array().as_ptr() );
        }
    }

    pub fn set_rgb_by_name( &self, name:&str, value:&color::RGB ) {
        self.set_rgb( self.get_uniform_location(name), value );
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram( self.id() ); }
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn from_shaders( shaders: &[Shader] ) -> Result<Self, Error> {
        unsafe {
            let id:GLuint = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader( id, shader.id() );
            }

            gl::LinkProgram( id );

            let mut link_status:GLint = 1;
            gl::GetProgramiv( id, gl::LINK_STATUS, &mut link_status );

            if FAILED == link_status {
                return Err(
                    Error::Linking( opengl_fn::gl_error_linking( id ) )
                )
            }

            for shader in shaders {
                gl::DetachShader( id, shader.id() );
            }

            return Ok( Self { id } );

        }
    }

}

impl Drop for ShaderProgram {
    fn drop( &mut self ) {
        unsafe { gl::DeleteProgram( self.id() ) }
    }
}

pub struct Shader {
    id: GLuint,
}

impl Shader {

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn vert_from_source( src: &c_string::CStr ) -> Result<Self, Error> {
        Self::from_source(src, ShaderKind::Vertex)
    }

    pub fn frag_from_source( src: &c_string::CStr ) -> Result<Self, Error> {
        Self::from_source(src, ShaderKind::Fragment)
    }

    fn from_source( src: &c_string::CStr, kind:ShaderKind ) -> Result<Self, Error> {

        let id:GLuint = Self::create_shader( kind );
        unsafe {
            gl::ShaderSource(
                id, 1,
                &src.as_ptr(), core::ptr::null()
            );
            gl::CompileShader( id );
        }

        let mut compile_status:GLint = 1;
        unsafe {
            gl::GetShaderiv( id, gl::COMPILE_STATUS, &mut compile_status );
        }

        if FAILED == compile_status {
            return Err(
                Error::Compilation( opengl_fn::gl_error_compilation( id ))
            );
        }

        return Ok( Self{ id } );

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
            gl::DeleteShader( self.id() );
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
