use crate::{ cstr::*, debugging::{log, Error}, Rc };
use gl::types::*;
use fmath::types::*;
use super::{ Sampler, ShaderProgram, Texture };
use core::fmt;

pub struct UniformInfo {
    name:     CString,
    kind:     GLenum ,
    location: GLint  ,
}

impl UniformInfo {

    pub fn new( name:CString, kind:GLenum, location:GLint ) -> Self {
        Self { name, kind, location }
    }

    pub fn name(&self)     -> &CStr  { &self.name    }
    pub fn kind(&self)     -> GLenum { self.kind     }
    pub fn location(&self) -> GLint  { self.location }

    pub fn generate_values( uniforms:&Vec<Self> ) -> Vec<Uniform> {
        let mut buffer = Vec::with_capacity( uniforms.len() );
        for uniform in uniforms.iter() {
            let uniform_value = Uniform::new( uniform.kind(), uniform.location() );
            buffer.push( uniform_value );
        }
        buffer
    }

}

impl fmt::Display for UniformInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Uniform {} \"{}\": {}",
            self.location(),
            self.name().to_str().unwrap(),
            self.kind()
        )
    }
}

#[derive(Debug, Clone)]
// TODO: Array uniforms are counted as only one uniform!
pub enum Uniform {
    Float32( f32, GLint ),
    // TODO: f64, i32, u32, bool variants
    Float32Vec2( Vector2, GLint ),
    Float32Vec3( Vector3, GLint ),
    Float32Vec4( Vector4, GLint ),

    Float64(  f64, GLint ),
    Int32  (  i32, GLint ),
    UInt32 (  u32, GLint ),
    Bool   ( bool, GLint ),

    // TODO: other matrices nxm, f64 variants
    Float32Mat3( Matrix3x3, GLint ),
    Float32Mat4( Matrix4x4, GLint ),

    // TODO: other sampler variants
    Sampler2D( ( Rc<Texture>, Sampler), GLint ),

    /// placeholder for unimplemented types
    None,
}

impl Uniform {
    pub fn find_uniform_mut_by_location( uniforms:&mut Vec<Self>, location:GLint ) -> Result<&mut Self, Error> {
        match uniforms.iter_mut().find( |u| u.location() == location )
        {
            Some(res) => Ok(res),
            None => Err( Error::UniformNotFound( format!("Uniform at location {} not found!", location) ) ),
        }
    }

    pub fn find_uniform_by_location( uniforms:&Vec<Self>, location:GLint ) -> Result<&Self, Error> {
        match uniforms.iter().find( |u| u.location() == location )
        {
            Some(res) => Ok(res),
            None => Err( Error::UniformNotFound( format!("Uniform at location {} not found!", location) ) ),
        }
    }

    pub fn find_uniform_mut_by_name<'a>(
        uniforms:&'a mut Vec<Self>, shader:&Rc<ShaderProgram>, name:&str
    ) -> Result<&'a mut Self, Error> {
        let location = shader.get_uniform_location(name);
        match uniforms.iter_mut().find( |u| u.location() == location )
        {
            Some(res) => Ok(res),
            None => Err( Error::UniformNotFound( format!("Uniform \"{}\" not found!", name) ) ),
        }
    }

    pub fn find_uniform_by_name<'a>(
        uniforms:&'a Vec<Self>, shader:&Rc<ShaderProgram>, name:&str
    ) -> Result<&'a Self, Error> {
        let location = shader.get_uniform_location(name);
        match uniforms.iter().find( |u| u.location() == location )
        {
            Some(res) => Ok(res),
            None => Err( Error::UniformNotFound( format!("Uniform \"{}\" not found!", name) ) ),
        }
    }

    pub fn new( g:GLenum, location:GLint ) -> Self {
        match g {
            gl::FLOAT      => Self::Float32    (              0.0f32, location ),
            gl::FLOAT_VEC2 => Self::Float32Vec2( Vector2::new_zero(), location ),
            gl::FLOAT_VEC3 => Self::Float32Vec3( Vector3::new_zero(), location ),
            gl::FLOAT_VEC4 => Self::Float32Vec4( Vector4::new_zero(), location ),

            gl::DOUBLE       => Self::Float64    (                0.0f64, location ),
            gl::INT          => Self::Int32      (                  0i32, location ),
            gl::UNSIGNED_INT => Self::UInt32     (                  0u32, location ),
            gl::BOOL         => Self::Bool       (                 false, location ),
            gl::FLOAT_MAT3   => Self::Float32Mat3( Matrix3x3::new_zero(), location ),
            gl::FLOAT_MAT4   => Self::Float32Mat4( Matrix4x4::new_zero(), location ),

            gl::SAMPLER_2D => Self::Sampler2D( ( Texture::empty(), Sampler::empty() ), location ),

            _ => Self::None,
        }
    }

    pub fn location( &self ) -> GLint {
        match self {
            Self::Float32    ( _, location ) |
            Self::Float32Vec2( _, location ) |
            Self::Float32Vec3( _, location ) |
            Self::Float32Vec4( _, location ) |
            Self::Float64    ( _, location ) |
            Self::Int32      ( _, location ) |
            Self::UInt32     ( _, location ) |
            Self::Bool       ( _, location ) |
            Self::Float32Mat3( _, location ) |
            Self::Float32Mat4( _, location ) |
            Self::Sampler2D  ( _, location ) => *location,

            Self::None => -1,
        }
    }

    pub fn set_f32(&mut self, v:f32) {
        match self {
            Self::Float32(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign f32 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_vector2(&mut self, v:Vector2) {
        match self {
            Self::Float32Vec2(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign Vector2 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_vector3(&mut self, v:Vector3) {
        match self {
            Self::Float32Vec3(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign Vector3 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_vector4(&mut self, v:Vector4) {
        match self {
            Self::Float32Vec4(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign Vector3 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_rgb(&mut self, v:color::RGB) {
        self.set_vector3( v.as_vector3() )
    }

    pub fn set_rgba(&mut self, v:color::RGB) {
        self.set_vector4( v.as_vector4() )
    }

    pub fn set_f64(&mut self, v:f64) {
        match self {
            Self::Float64(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign f64 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_i32(&mut self, v:i32) {
        match self {
            Self::Int32(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign i32 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_u32(&mut self, v:u32) {
        match self {
            Self::UInt32(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign u32 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_bool(&mut self, v:bool) {
        match self {
            Self::Bool(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign bool to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_matrix3x3(&mut self, v:Matrix3x3) {
        match self {
            Self::Float32Mat3(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign Matrix3x3 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_matrix4x4(&mut self, v:Matrix4x4) {
        match self {
            Self::Float32Mat4(value,_) => { *value = v; }
            _ => {
                log(
                    &format!("Attempted to assign Matrix4x4 to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn set_sampler2d(&mut self, v:( Rc<Texture>, Sampler)) {
        match self {
            Self::Sampler2D( value, _ ) => { *value = v },
            _ => {
                log(
                    &format!("Attempted to assign Sampler to Uniform of type {}!", self.type_name()),
                    "Uniform Value | Warning"
                )
            }
        }
    }

    pub fn send_if_dirty(&self, dirty:bool) {
        // some uniforms need to be sent regardless of if they're dirty or not
        match self {
            Self::Sampler2D(_, _) => { self.send_to_gl() }
            _ => {
                if dirty { self.send_to_gl() }
            }
        }
    }

    pub fn send_to_gl(&self) {
        match self {
            Uniform::Float32    (v, loc) => unsafe{
                gl::Uniform1f( *loc, *v );
            },
            Uniform::Float32Vec2(v, loc) => unsafe{
                gl::Uniform2fv( *loc, 1, v.as_ptr() );
            },
            Uniform::Float32Vec3(v, loc) => unsafe{
                gl::Uniform3fv( *loc, 1, v.as_ptr() );
            },
            Uniform::Float32Vec4(v, loc) => unsafe{
                gl::Uniform4fv( *loc, 1, v.as_ptr() );
            },
            Uniform::Float64    (v, loc) => unsafe{
                gl::Uniform1d( *loc, *v );
            },
            Uniform::Int32      (v, loc) => unsafe{
                gl::Uniform1i( *loc, *v );
            },
            Uniform::UInt32     (v, loc) => unsafe{
                gl::Uniform1ui( *loc, *v );
            },
            Uniform::Bool       (v, loc) => unsafe{
                let val = if *v { 1 } else { 0 };
                gl::Uniform1ui( *loc, val );
            },
            Uniform::Float32Mat3(v, loc) => unsafe{
                gl::UniformMatrix3fv( *loc, 1, gl::FALSE, v.as_ptr() );
            },
            Uniform::Float32Mat4(v, loc) => unsafe{
                gl::UniformMatrix4fv( *loc, 1, gl::FALSE, v.as_ptr() );
            },
            Uniform::Sampler2D  (v, loc) => unsafe{
                v.0.use_texture( &v.1, *loc );
                gl::Uniform1i( *loc, *v.1.id() );
            },
            _ => {
                log(
                    &format!( "Sent no data to OpenGL." ),
                    &format!( "{}", self )
                )
            },
        }
    }

    pub fn type_name(&self) -> &str {
        match self {
            Uniform::Float32    (_, _) => "f32",
            Uniform::Float32Vec2(_, _) => "f32 Vector2",
            Uniform::Float32Vec3(_, _) => "f32 Vector3",
            Uniform::Float32Vec4(_, _) => "f32 Vector4",
            Uniform::Float64    (_, _) => "f64",
            Uniform::Int32      (_, _) => "i32",
            Uniform::UInt32     (_, _) => "u32",
            Uniform::Bool       (_, _) => "bool",
            Uniform::Float32Mat3(_, _) => "f32 Matrix3x3",
            Uniform::Float32Mat4(_, _) => "f32 Matrix4x4",
            Uniform::Sampler2D  (_, _) => "Sampler2D",
            Uniform::None              => "None",
        }
    }

    pub fn format_data(&self) -> String {
        match self {
            Uniform::Float32    (v, _)       => format!("{:.3}", v),
            Uniform::Float32Vec2(v, _)   => format!("{}", v),
            Uniform::Float32Vec3(v, _)   => format!("{}", v),
            Uniform::Float32Vec4(v, _)   => format!("{}", v),
            Uniform::Float64    (v, _)       => format!("{:.3}", v),
            Uniform::Int32      (v, _)       => format!("{}", v), 
            Uniform::UInt32     (v, _)       => format!("{}", v), 
            Uniform::Bool       (v, _)      => format!("{}", v), 
            Uniform::Float32Mat3(v, _) => format!("{:?}", v.as_array()),
            Uniform::Float32Mat4(v, _) => format!("{:?}", v.as_array()),
            Uniform::Sampler2D  (v, _) => format!("{} {}", v.0, v.1),
            Uniform::None => "None".to_owned(),
        }
    }

}

impl fmt::Display for Uniform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Uniform {:3}: {} {}", self.location(), self.type_name(), self.format_data() )
    }
}
