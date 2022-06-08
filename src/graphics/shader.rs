use gl::types::*;
use core::fmt;
use crate::{ cstr::*, Rc, debugging::{ Error, log } };
use super::uniform::{ UniformInfo, Uniform };

pub struct ShaderProgram {
    handle: GLuint,
    uniform_info: Vec<UniformInfo>,
}

impl ShaderProgram {

    pub fn from_shaders( shaders: &[Shader] ) -> Result<Rc<Self>, Error> {
        unsafe {
            let handle:GLuint = gl::CreateProgram();

            for shader in shaders.iter() {
                gl::AttachShader( handle, shader.handle() );
            }

            gl::LinkProgram( handle );

            let mut link_status:GLint = 1;
            gl::GetProgramiv( handle, gl::LINK_STATUS, &mut link_status );

            if link_status == FAILED {
                return Err( linking_error( handle ) );
            }

            log(
                "Successfully linked shaders.",
                "Shader Linker"
            );

            for shader in shaders.iter() {
                gl::DetachShader( handle, shader.handle() );
            }

            return Ok( Rc::new( Self { handle, uniform_info: Self::gen_uniform_info( handle ) } ) );

        }
    }

    fn gl_get_uniform_count( handle:GLuint ) -> usize {
        unsafe {
            let mut count = 0;
            gl::GetProgramiv( handle, gl::ACTIVE_UNIFORMS, &mut count );
            if count < 0 { 0 }
            else { count as usize }
        }
    }

    fn gl_get_uniform_location( handle:GLuint, name:&CStr ) -> GLint {
        unsafe { gl::GetUniformLocation( handle, name.as_ptr() as *const GLchar ) }
    }

    fn gen_uniform_info( handle:GLuint ) -> Vec<UniformInfo> {
        unsafe {
            let count = Self::gl_get_uniform_count( handle );
            if count == 0 {
                log(
                    &format!( "Generated no uniforms for shader {}.", handle ),
                    "Shader Program"
                );
                return Vec::new();
            }
            let mut result:Vec<UniformInfo> = Vec::with_capacity( count );

            for idx in 0..count {
                let mut data_size = 0;
                let mut kind = 0;
                let mut name_buffer_len = 0;
                let buffer_size = 128;

                let mut name_buffer = vec![0u8;buffer_size as usize];

                gl::GetActiveUniform(
                    handle, idx as GLuint,
                    buffer_size, &mut name_buffer_len,
                    &mut data_size, &mut kind,
                    name_buffer.as_ptr() as *mut GLchar
                );
                name_buffer.truncate( (name_buffer_len + 1) as usize );
                let name = CString::from_vec_with_nul_unchecked( name_buffer );
                let location = Self::gl_get_uniform_location(handle, &name);

                if location == INVALID_LOCATION { continue; }

                let uniform = UniformInfo::new( name, kind, location );

                result.push( uniform );
            }

            result.shrink_to_fit();

            log(
                &format!( "Generated {} uniforms for shader {}.", result.len(), handle ),
                "Shader Program"
            );

            result
        }
    }

    fn gl_get_uniform_block_index( handle:GLuint, name:&CStr ) -> GLuint {
        unsafe {
            gl::GetUniformBlockIndex( handle, name.as_ptr() as *const GLchar )
        }
    }

    pub fn use_program(&self) { unsafe { gl::UseProgram( self.handle() ); } }

    pub fn generate_uniforms(&self) -> ( Vec<Uniform>, Vec<bool> ) {
        let uniforms = UniformInfo::generate_values(&self.uniform_info);
        let dirty = vec![true;uniforms.len()];
        ( uniforms, dirty )
    }

    pub fn handle(&self) -> GLuint { self.handle }

    pub fn uniform_count(&self) -> usize { self.uniform_info.len() }

    pub fn get_uniform_location(&self, name:&str) -> GLint {
        let cname = CString::new( name ).unwrap();
        let result = Self::gl_get_uniform_location( self.handle(), &cname );
        #[cfg(debug_assertions)]
        if result == INVALID_LOCATION {
            log(
                &format!( "Uniform \"{}\" not found!", name ),
                &format!( "Shader {} get_uniform_location()", self.handle() )
            )
        }

        result
    }

    pub fn get_uniform_block_index(&self, name:&str) -> GLuint {
        let cname = CString::new( name ).unwrap();
        let result = Self::gl_get_uniform_block_index( self.handle(), &cname );
        #[cfg(debug_assertions)]
        if result == gl::INVALID_INDEX {
            log(
                &format!( "Uniform Block \"{}\" not found!", name ),
                &format!( "Shader {} get_uniform_block_index()", self.handle() )
            )
        }

        result
    }

    pub fn bind_uniform_block( &self, block_index:GLuint, block_binding:GLuint ) {
        unsafe {
            gl::UniformBlockBinding(
                self.handle(), block_index,
                block_binding
            );
        }
    }

    pub fn bind_uniform_block_by_name( &self, name:&str, block_binding:GLuint ) {
        let idx = self.get_uniform_block_index( name );
        self.bind_uniform_block( idx, block_binding );
    }

}

impl Drop for ShaderProgram {
    fn drop( &mut self ) {
        unsafe { gl::DeleteProgram( self.handle() ) }
    }
}

impl fmt::Display for ShaderProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = String::new();
        for uniform in self.uniform_info.iter() {
            buffer.push_str( &format!( "   {}\n", uniform ) )
        }
        write!( f, "Shader {}\n{}", self.handle(), buffer )
    }
}

pub struct Shader { handle:GLuint }

impl Shader {

    pub fn handle(&self) -> GLuint { self.handle }

    pub fn vert_from_source( src:&CStr ) -> Result<Self, Error> {
        Self::from_source(src, ShaderKind::Vertex)
    }

    pub fn frag_from_source( src:&CStr ) -> Result<Self, Error> {
        Self::from_source(src, ShaderKind::Fragment)
    }

    pub fn from_source( src:&CStr, kind:ShaderKind ) -> Result<Self, Error> {
        unsafe {
            let handle:GLuint = gl::CreateShader( kind as GLenum );
            gl::ShaderSource(
                handle, 1,
                &src.as_ptr(), core::ptr::null()
            );
            gl::CompileShader( handle );
            
            let mut compile_status:GLint = 1;
            gl::GetShaderiv( handle, gl::COMPILE_STATUS, &mut compile_status );

            if compile_status == FAILED { return Err( compilation_error( handle ) ); }
            else {
                log(
                    format!("Successfully compiled {} shader.", kind ).as_str(),
                    "Shader Compiler"
                );
                return Ok( Self{ handle } );
            }
        }
    }

}

impl Drop for Shader {
    fn drop( &mut self ) {
        unsafe { gl::DeleteShader( self.handle() ) }
    }
}

/// I dont want my fn to accept just any GLenum
#[derive(Clone, Copy)]
pub enum ShaderKind {
    Vertex   = 0x8B31,
    Fragment = 0x8B30,
}

impl fmt::Display for ShaderKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShaderKind::Vertex   => write!( f, "Vertex" ),
            ShaderKind::Fragment => write!( f, "Fragment" ),
        }
    }
}

const FAILED:GLint = 0;

pub fn compilation_error( shader_handle:GLuint ) -> Error {
    unsafe {
        let mut len = 0;
        gl::GetShaderiv( shader_handle, gl::INFO_LOG_LENGTH, &mut len );
        let message_buffer = CString::whitespace_buffer( len as usize );
        gl::GetShaderInfoLog(
            shader_handle, len,
            core::ptr::null_mut(),
            message_buffer.as_ptr() as *mut GLchar
        );
        match message_buffer.to_str() {
            Ok( message ) => Error::ShaderCompiler( message.replace("\n", "").to_owned() ),
            Err( e ) => Error::UTF8( format!( "{}", e ) ),
        }
    }
}

pub fn linking_error( shader_handle:GLuint ) -> Error {
    unsafe {
        let mut len:GLint = 0;
        gl::GetProgramiv( shader_handle, gl::INFO_LOG_LENGTH, &mut len );
        let message_buffer = CString::whitespace_buffer( len as usize );
        gl::GetProgramInfoLog(
            shader_handle, len,
            core::ptr::null_mut(),
            message_buffer.as_ptr() as *mut GLchar
        );
        match message_buffer.to_str() {
            Ok( message ) => Error::ShaderCompiler( message.replace("\n", "").to_owned() ),
            Err( e ) => Error::UTF8( format!( "{}", e ) ),
        }
    }
}

pub fn shader_parser( src:&str ) -> Result<[Shader;2], Error> {

    // PPD - Pre-processor directive
    const PPD_VERTEX:&str   = "#vertex";
    const PPD_FRAGMENT:&str = "#fragment";

    // split into lines
    let lines:Vec<&str> = src.split('\n').collect();

    let mut vert_buffer:Vec<&str> = Vec::new();
    let mut frag_buffer:Vec<&str> = Vec::new();

    let mut shader_kind = ParseKind::None;

    for line in lines.iter() {
        // skip empty
        if line.is_empty() { continue; }

        if line.contains( PPD_VERTEX ) {
            shader_kind = ParseKind::Vertex;
            continue;
        } else if line.contains( PPD_FRAGMENT ) {
            shader_kind = ParseKind::Fragment;
            continue;
        }

        match shader_kind {
            ParseKind::Vertex => {
                vert_buffer.push( line );
            },
            ParseKind::Fragment => {
                frag_buffer.push( line );
            },
            _ => continue,
        }
    }

    if vert_buffer.is_empty() || frag_buffer.is_empty() {
        return Err( Error::ShaderParse( "Shader is not formatted properly!".to_owned() ) );
    }

    log(
        "Successfully parsed Vertex and Fragment shaders.",
        "Shader Parser"
    );

    let vert_src = CString::new( vert_buffer.join("\n") )
        .map_err( |e| Error::CStringNul( format!( "{}", e ) ) )?;
    let vert = Shader::vert_from_source( &vert_src )?;

    let frag_src = CString::new( frag_buffer.join("\n") )
        .map_err( |e| Error::CStringNul( format!( "{}", e ) ) )?;
    let frag = Shader::frag_from_source( &frag_src )?;

    Ok( [ vert, frag, ] )

}

enum ParseKind {
    Vertex,
    Fragment,
    None,
}

const INVALID_LOCATION:GLint = -1;

pub fn null_shader() -> Rc<ShaderProgram> {
    thread_local!(
        static NULL_SHADER: Rc<ShaderProgram> =
            crate::resources::load_shader_program("blinn-phong").unwrap()
    );
    NULL_SHADER.with( |s| s.clone() )
}
