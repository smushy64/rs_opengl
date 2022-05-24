use crate::Rc;
use super::{
    ShaderProgram,
    MaterialUniform,
};
use gl::types::*;

use core::{
    ops::{
        Index, IndexMut
    }, fmt
};

/// Uniforms indexable with usize or its name as &str
/// 
/// Indexing with &str is slow!
#[allow(dead_code)]
pub struct Material {
    name:       String,
    shader_ref: Rc<ShaderProgram>,
    uniforms:   Vec<MaterialUniform>
}

impl Material {

    pub fn new( name:&str, shader:Rc<ShaderProgram> ) -> Self {
        let material_uniforms = shader.generate_material_uniforms();
        Self {
            name: String::from( name ),
            shader_ref: shader,
            uniforms: material_uniforms
        }
    }

    pub fn clone( &self ) -> Self {
        Self {
            name: format!( "{} 0", self.get_name() ),
            shader_ref: self.shader_ref.clone(),
            uniforms: self.uniforms.clone()
        }
    }

    pub fn get_name( &self )   -> &str           { &self.name }
    pub fn get_shader( &self ) -> &ShaderProgram { &self.shader_ref }
    pub fn get_uniform_location( &self, name:&str ) -> Option<usize> {
        for (idx, uniform) in self.uniforms.iter().enumerate() {
            if name == uniform.name() {
                return Some( idx );
            }
        }
        return None;
    }

    pub fn activate_shader( &self ) { self.shader_ref.use_program(); }

    pub fn use_material(&self) {
        self.activate_shader();
        for ( idx, uniform ) in self.uniforms.iter().enumerate() {
            uniform.send_value_to_shader( idx as GLint )
        }
    }

}

impl Index<usize> for Material {
    type Output = MaterialUniform;

    fn index(&self, idx: usize) -> &MaterialUniform {
        &self.uniforms[idx]
    }
}

impl Index<&str> for Material {
    type Output = MaterialUniform;

    fn index(&self, name: &str) -> &MaterialUniform {
        match self.get_uniform_location( name ) {
            Some(u) => &self[u],
            None => panic!( "Uniform \"{}\" cannot be found!", name ),
        }
    }
}

impl IndexMut<usize> for Material {
    fn index_mut(&mut self, idx: usize) -> &mut MaterialUniform {
        &mut self.uniforms[idx]
    }
}

impl IndexMut<&str> for Material {
    fn index_mut(&mut self, name: &str) -> &mut MaterialUniform {
        match self.get_uniform_location( name ) {
            Some(u) => &mut self[u],
            None => panic!( "Uniform \"{}\" cannot be found!", name ),
        }
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = {
            let mut buffer = String::new();
            for (idx, uniform) in self.uniforms.iter().enumerate() {
                buffer.push_str( &format!("     Uniform {:3} | {}\n", idx, uniform) )
            }
            buffer
        };
        write!( f, "Material \"{}\"\n{}", self.name, msg )
    }
}
