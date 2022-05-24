use std::rc::Rc;
use crate::{
    Mesh, Material,
};

pub struct Model {
    meshes:Rc<Vec<Mesh>>,
    pub material:Material
}

impl Model {
    pub fn new( meshes:Rc<Vec<Mesh>>, material:Material ) -> Self {
        Self { meshes, material }
    }

    pub fn render( &self ) {
        self.material.use_material();
        for mesh in self.meshes.iter() {
            mesh.render();
        }
    }
}