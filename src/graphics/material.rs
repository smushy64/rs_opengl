use gl::types::*;
use crate::{ Rc, debugging::Error };
use super::{ ShaderProgram, Uniform, null_shader };
use core::{ fmt, ops::{ Index, IndexMut } };

pub struct Material {
    shader: Rc<ShaderProgram>,
    uniforms: ( Vec<Uniform>, Vec<bool> ),
}

impl Material {

    pub fn new( shader:Rc<ShaderProgram> ) -> Self {
        let uniforms = shader.generate_uniforms();
        Self { shader, uniforms }
    }

    pub fn new_null() -> Self {
        Self::new( null_shader() )
    }

    pub fn clone_from( m:&Self ) -> Self {
        Self { shader: m.shader.clone(), uniforms: m.uniforms.clone() }
    }

    pub fn shader(&self)   -> &Rc<ShaderProgram> { &self.shader }
    pub fn uniforms(&self) -> &Vec<Uniform>  { &self.uniforms.0 }

    pub fn use_shader(&self) { self.shader.use_program() }
    pub fn send_uniforms_to_gl(&mut self) {
        for (idx, uniform) in self.uniforms.0.iter().enumerate() {
            let dirty_flag = &mut self.uniforms.1[idx];
            uniform.send_if_dirty( *dirty_flag );
            *dirty_flag = false;
        }
    }
    pub fn send_all_uniforms_to_gl(&self) {
        for uniform in self.uniforms.0.iter() {
            uniform.send_to_gl();
        }
    }
    pub fn use_material(&mut self) { self.use_shader(); self.send_uniforms_to_gl(); }

    pub fn get_uniform_location( &self, name:&str ) -> usize {
        let result = self.shader.get_uniform_location(name);
        if result < 0 { panic!( "Uniform \"{}\" not found!", name ) }
        result as usize
    }

    pub fn get_uniform_by_name( &self, name:&str ) -> Result<&Uniform, Error> {
        Uniform::find_uniform_by_name( &self.uniforms.0, &self.shader, name )
    }

    pub fn get_uniform_by_location( &self, location:GLint ) -> Result<&Uniform, Error> {
        Uniform::find_uniform_by_location( &self.uniforms.0, location )
    }

    pub fn get_uniform_mut_by_name( &mut self, name:&str ) -> Result<&mut Uniform, Error> {
        let loc = self.shader.get_uniform_location(name);
        match self.uniforms.0.iter().position( |u| u.location() == loc ) {
            Some(idx) => {
                self.uniforms.1[idx] = true;
                Ok( &mut self.uniforms.0[idx] )
            },
            None => Err( Error::UniformNotFound( format!("Uniform \"{}\" not found!", name) ) ),
        }
    }

    pub fn get_uniform_mut_by_location( &mut self, location:GLint ) -> Result<&mut Uniform, Error> {
        match self.uniforms.0.iter().position( |u| u.location() == location ) {
            Some(idx) => {
                self.uniforms.1[idx] = true;
                Ok( &mut self.uniforms.0[idx] )
            },
            None => Err( Error::UniformNotFound( format!("Uniform at location {} not found!", location) ) ),
        }
    }

}

impl Index<usize> for Material {
    type Output = Uniform;

    fn index( &self, index:usize ) -> &Uniform {
        match self.get_uniform_by_location( index as GLint ) {
            Ok(res) => res,
            Err(e) => panic!( "{}", e ),
        }
    }
}

impl Index<&str> for Material {
    type Output = Uniform;

    fn index( &self, name:&str ) -> &Uniform {
        match self.get_uniform_by_name( name ) {
            Ok(res) => res,
            Err(e) => panic!( "{}", e ),
        }
    }
}

impl IndexMut<usize> for Material {
    fn index_mut( &mut self, index:usize ) -> &mut Uniform {
        match self.get_uniform_mut_by_location( index as GLint ) {
            Ok(res) => res,
            Err(e) => panic!( "{}", e ),
        }
    }
}

impl IndexMut<&str> for Material {
    fn index_mut( &mut self, name:&str ) -> &mut Uniform {
        match self.get_uniform_mut_by_name( name ) {
            Ok(res) => res,
            Err(e) => panic!( "{}", e ),
        }
    }
}

impl fmt::Display for Material {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut uniform_info_buffer = String::new();
        for uniform in self.uniforms.0.iter() {
            uniform_info_buffer.push_str( &format!( "   {}\n", uniform ) )
        }
        write!( f, "Material | Shader: {} \n{}", self.shader().handle(), uniform_info_buffer )
    }
}
