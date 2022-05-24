use gl::types::*;
use fmath::types::*;

use crate::Rc;

use super::{
    Texture, Sampler
};

pub struct UniformInfo {
    pub(super) name:   String,
    pub(super) kind:   GLenum,
}

impl UniformInfo {
    pub(super) fn new(
        name:String,
        kind:GLenum,
    ) -> Self {
        Self { name, kind }
    }

    pub fn name(&self)   -> &str    { &self.name   }
    pub fn kind(&self)   -> &GLenum { &self.kind   }

    pub fn generate_material_uniforms( uniforms:&Vec<Self> ) -> Vec<MaterialUniform> {
        let mut material_uniforms = Vec::with_capacity( uniforms.len() );
        for uniform in uniforms.iter() {
            material_uniforms.push(
                MaterialUniform::new(
                    String::from(uniform.name()),
                    uniform.kind().clone()
                )
            )
        }
        material_uniforms
    }

}

pub struct MaterialUniform {
    name:  String,
    value: UniformValue,
}

impl MaterialUniform {
    pub fn new( name:String, kind_glenum:GLenum ) -> Self {
        Self { name, value: UniformValue::from_glenum( kind_glenum ) }
    }

    pub fn name( &self )  -> &str          { &self.name  }
    pub fn value( &self ) -> &UniformValue { &self.value }

    pub fn set_u32( &mut self, value:u32 ) {
        match self.value {
            UniformValue::U32(_) => { self.value = UniformValue::U32(value); },
            _ => {}
        }
    }

    pub fn set_i32( &mut self, value:i32 ) {
        match self.value {
            UniformValue::I32(_) => { self.value = UniformValue::I32(value); },
            _ => {}
        }
    }

    pub fn set_texture( &mut self, value:( Rc<Texture>, Sampler ) ) {
        match self.value {
            UniformValue::Texture(_) => { self.value = UniformValue::Texture( value ); },
            _ => {}
        }
    }

    pub fn set_f32( &mut self, value:f32 ) {
        match self.value {
            UniformValue::F32(_) => { self.value = UniformValue::F32(value); },
            _ => {}
        }
    }

    pub fn set_vector2( &mut self, value:Vector2 ) {
        match self.value {
            UniformValue::Vector2(_) => { self.value = UniformValue::Vector2(value); },
            _ => {}
        }
    }

    pub fn set_vector3( &mut self, value:Vector3 ) {
        match self.value {
            UniformValue::Vector3(_) => { self.value = UniformValue::Vector3(value); },
            _ => {}
        }
    }

    pub fn set_vector4( &mut self, value:Vector4 ) {
        match self.value {
            UniformValue::Vector4(_) => { self.value = UniformValue::Vector4(value); },
            _ => {}
        }
    }

    pub fn set_rgb( &mut self, value:color::RGB ) {
        self.set_vector3( value.as_vector3() )
    }

    pub fn set_rgba( &mut self, value:color::RGB ) {
        self.set_vector4( value.as_vector4() )
    }

    pub fn set_matrix3( &mut self, value:Matrix3x3 ) {
        match self.value {
            UniformValue::Matrix3x3(_) => { self.value = UniformValue::Matrix3x3(value); },
            _ => {}
        }
    }

    pub fn set_matrix4( &mut self, value:Matrix4x4 ) {
        match self.value {
            UniformValue::Matrix4x4(_) => { self.value = UniformValue::Matrix4x4(value); },
            _ => {}
        }
    }

    pub fn send_value_to_shader( &self, id:GLint ) {
        self.value.send_to_shader( id );
    }

}

impl core::fmt::Display for MaterialUniform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}:   {}", self.name(), self.value() )
    }
}

pub enum UniformValue {

    F32(f32),

    U32(u32),
    I32(i32),

    Vector2(Vector2),
    Vector3(Vector3),
    Vector4(Vector4),

    Matrix3x3(Matrix3x3),
    Matrix4x4(Matrix4x4),

    Texture( ( Rc<Texture>, Sampler ) ),

    None

}

impl UniformValue {

    pub fn send_to_shader( &self, uniform_location:GLint ) {
        match self {
            UniformValue::F32(v) => unsafe{ gl::Uniform1f( uniform_location, *v );  },
            UniformValue::U32(v) => unsafe{ gl::Uniform1ui( uniform_location, *v ); },
            UniformValue::I32(v) => unsafe{ gl::Uniform1i( uniform_location, *v ); },
            UniformValue::Vector2(v) => unsafe{
                gl::Uniform2fv( uniform_location, 1, v.as_ptr() );
            },
            UniformValue::Vector3(v) => unsafe{
                gl::Uniform3fv( uniform_location, 1, v.as_ptr() );
            },
            UniformValue::Vector4(v) => unsafe{
                gl::Uniform4fv( uniform_location, 1, v.as_ptr() );
            },
            UniformValue::Matrix3x3(v) => unsafe{
                gl::UniformMatrix3fv(
                    uniform_location, 1,
                    gl::FALSE, v.as_ptr()
                );
            },
            UniformValue::Matrix4x4(v) => unsafe{
                gl::UniformMatrix4fv(
                    uniform_location, 1,
                    gl::FALSE, v.as_ptr()
                );
            },
            UniformValue::Texture((tex, sampler)) => {
                tex.use_texture( &sampler, uniform_location );
            },
            UniformValue::None => {},
        }
    }

    pub fn from_glenum( glenum:GLenum ) -> Self {
        match glenum {
            gl::FLOAT        => Self::F32( 0.0 ),
            
            gl::INT          => Self::I32( 0 ),
            gl::UNSIGNED_INT => Self::U32( 0 ),

            gl::FLOAT_VEC2 => Self::Vector2( Vector2::new_zero() ),
            gl::FLOAT_VEC3 => Self::Vector3( Vector3::new_zero() ),
            gl::FLOAT_VEC4 => Self::Vector4( Vector4::new_zero() ),
            
            gl::FLOAT_MAT3 => Self::Matrix3x3( Matrix3x3::new_zero() ),
            gl::FLOAT_MAT4 => Self::Matrix4x4( Matrix4x4::new_zero() ),
            
            gl::SAMPLER_2D => Self::Texture( ( Rc::new( Texture::empty() ), Sampler::empty() ) ),

            _ => Self::None
        }
    }

}

impl core::fmt::Display for UniformValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            UniformValue::F32(v)               => format!("( {:7.3} )", v),
            UniformValue::U32(v)               => format!("( {} )", v),
            UniformValue::I32(v)               => format!("( {} )", v),
            UniformValue::Vector2(v)       => format!("{}", v),
            UniformValue::Vector3(v)       => format!("{}", v),
            UniformValue::Vector4(v)       => format!("{}", v),
            UniformValue::Matrix3x3(v)   => format!("( {:?} )", v.as_array()),
            UniformValue::Matrix4x4(v)   => format!("( {:?} )", v.as_array()),
            UniformValue::Texture((_,v)) => format!( "{}", v ),
            _ => format!(""),
        };
        write!(f, "{}", v)
    }
}
