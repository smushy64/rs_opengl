use gl::types::*;
use fmath::types::*;
use crate::{ c_string, Rc };

use super::{
    uniform::UniformInfo,
    MaterialUniform,
    Sampler
};

pub struct ShaderProgram {
    handle: GLuint,
    uniforms: Vec<UniformInfo>,
}

impl ShaderProgram {

    pub fn from_shaders( shaders: &[Shader] ) -> Result<Rc<Self>, Error> {
        unsafe {
            let handle:GLuint = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader( handle, shader.handle() );
            }

            gl::LinkProgram( handle );

            let mut link_status:GLint = 1;
            gl::GetProgramiv( handle, gl::LINK_STATUS, &mut link_status );

            if FAILED == link_status {
                return Err( linking_error( handle ) );
            }

            for shader in shaders {
                gl::DetachShader( handle, shader.handle() );
            }

            return Ok(
                Rc::new( Self {
                    handle,
                    uniforms: Self::get_uniforms( handle )
                } )
            );

        }
    }

    fn get_uniforms( handle:GLuint ) -> Vec<UniformInfo> {
        unsafe {

            let mut uniform_count = 0;
            gl::GetProgramiv(
                handle,
                gl::ACTIVE_UNIFORMS,
                &mut uniform_count
            );
            let mut uniforms:Vec<UniformInfo> = Vec::with_capacity( uniform_count as usize );

            for i in 0..uniform_count {

                let mut uniform_size:GLint  = 0;
                let mut uniform_kind:GLenum = 0;
                let uniform_name_buffer_len:GLint = 255;

                let name = c_string::create_empty_c_string( ( uniform_name_buffer_len + 1 ) as usize );

                gl::GetActiveUniform(
                    handle, i as GLuint,
                    uniform_name_buffer_len, core::ptr::null_mut(),
                    &mut uniform_size, &mut uniform_kind,
                    name.as_ptr() as *mut GLchar
                );

                uniforms.push(
                    UniformInfo::new(
                        c_string::to_string( name ),
                        uniform_kind,
                    )
                );

            }

            return uniforms;

        }
    }

    pub fn generate_material_uniforms( &self ) -> Vec<MaterialUniform> {
        UniformInfo::generate_material_uniforms( &self.uniforms )
    }

    pub fn get_uniform_location( &self, name:&str ) -> GLint {
        let mut location = -1;
        for ( idx, uniform ) in self.uniforms.iter().enumerate() {
            if uniform.name() == name {
                location = idx as GLint;
                break;
            }
        }
        location
    }

    pub fn set_sampler( &self, loc:GLint, value:&Sampler ) {
        self.set_i32( loc, value.id() );
    }

    pub fn set_sampler_by_name( &self, name:&str, value:&Sampler ) {
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
        unsafe { gl::UseProgram( *self.handle() ); }
    }

    pub fn handle(&self)   -> &GLuint           { &self.handle }
    pub fn uniforms(&self) -> &Vec<UniformInfo> { &self.uniforms }

}

impl Drop for ShaderProgram {
    fn drop( &mut self ) {
        unsafe { gl::DeleteProgram( *self.handle() ) }
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
            return Err( compilation_error( handle ) );
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
    Parse(String),
}

impl Error {
    pub fn msg(&self) -> String {
        match self {
            Error::Linking( s )     => s.clone(),
            Error::Compilation( s ) => s.clone(),
            Error::Parse( s )       => s.clone(),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.msg())
    }
}

pub fn compilation_error( id:GLuint ) -> Error {
    unsafe {
        let mut len:GLint = 0;
        gl::GetShaderiv( id, gl::INFO_LOG_LENGTH, &mut len );
        let message = c_string::create_empty_c_string( len as usize );
        gl::GetShaderInfoLog(
            id, len,
            core::ptr::null_mut(),
            message.as_ptr() as *mut GLchar
        );
        return Error::Compilation(
            format!(
                "Shader Compilation Error: {}",
                message.to_string_lossy().into_owned()
            )
        );
    }
}

pub fn linking_error( id:GLuint ) -> Error {
    unsafe {
        let mut len:GLint = 0;
        gl::GetProgramiv( id, gl::INFO_LOG_LENGTH, &mut len );
        let message = c_string::create_empty_c_string( len as usize );
        gl::GetProgramInfoLog(
            id, len,
            core::ptr::null_mut(),
            message.as_ptr() as *mut GLchar
        );
        return Error::Linking(
            format!(
                "Shader Linking Error: {}",
                message.to_string_lossy().into_owned()
            )
        );
    }
}

pub fn shader_parser( shader_program:String ) -> Result<[Shader;2], Error> {

    // PPD - Pre-processor directive
    const PPD_VERTEX:&str   = "#vertex";
    const PPD_FRAGMENT:&str = "#fragment";

    // split into lines
    let parts:Vec<&str> = shader_program.split('\n').collect();

    let mut vert = String::new();
    let mut frag = String::new();

    let mut shader_kind = ParseKind::None;

    for part in parts.iter() {

        if part.contains( PPD_VERTEX ) {
            shader_kind = ParseKind::Vertex;
            continue;
        } else if part.contains( PPD_FRAGMENT ) {
            shader_kind = ParseKind::Fragment;
            continue;
        }

        match shader_kind {
            ParseKind::Vertex => {
                vert.push_str(part);
                vert.push('\n');
            },
            ParseKind::Fragment => {
                frag.push_str(part);
                frag.push('\n');
            },
            ParseKind::None => continue,
        }

    }

    if vert.is_empty() || frag.is_empty() {
        return Err(
            Error::Parse(format!("Shader Parse Error: Shader is not formatted properly!"))
        );
    }

    Ok(
        [
            Shader::vert_from_source(
                &c_string::CString::new( vert.as_str() )
                .map_err(|_|
                    Error::Parse(
                        format!("Shader Parse Error: File contains null character!")
                    )
                )?
            )?,
            Shader::frag_from_source(
                &c_string::CString::new( frag.as_str() )
                .map_err(|_|
                    Error::Parse(
                        format!("Shader Parse Error: File contains null character!")
                    )
                )?
            )?,
        ]
    )

}

enum ParseKind {
    Vertex,
    Fragment,
    None,
}
