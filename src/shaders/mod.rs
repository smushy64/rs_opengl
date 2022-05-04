use gl::types::*;
use super::c_string;
use super::opengl_fn;

#[derive(Clone)]
pub struct ShaderProgram {
    id: GLuint
}

impl ShaderProgram {

    pub fn get_uniforms( &self ) -> Vec<( c_string::CString, GLenum, GLint )> {
        unsafe {
            // count of uniforms
            let mut count:GLint = 0;
            gl::GetProgramiv(
                self.id(),
                gl::ACTIVE_UNIFORMS,
                &mut count
            );

            let mut uniforms:Vec<( c_string::CString, GLenum, GLint )> = Vec::new();

            let mut i:GLint = 0;
            while i < count {

                let mut variable_size = 0;
                let mut uniform_type:GLenum = 0;
                let name_buffer_len:GLint = 255;
                
                let mut name = c_string::create_empty_c_string(
                    (name_buffer_len + 1) as usize
                );

                gl::GetActiveUniform(
                    self.id(), i as GLuint,
                    name_buffer_len, core::ptr::null_mut(),
                    &mut variable_size, &mut uniform_type,
                    name.as_ptr() as *mut GLchar
                );

                name = c_string::format_c_string_uniform_name(name).unwrap();

                let id = gl::GetUniformLocation(
                    self.id(),
                    name.as_ptr() as *const GLchar
                );

                uniforms.push( (name, uniform_type, id) );

                i += 1;
            }

            return uniforms;

        }

    }

    pub fn get_uniform_location( &self, name:&str ) -> GLint {
        unsafe {
            let c_name = c_string::c_string_from_str(name).unwrap();
            let location =
                gl::GetUniformLocation( self.id() , c_name.as_ptr() as *const GLchar );
            return location;
        }
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

pub enum ShaderKind {
    Vertex, Fragment
}

const FAILED:GLint = 0;

#[derive(Debug)]
pub enum Error {
    Linking(String),
    Compilation(String),
}
