use std::rc::Rc;
use crate::{
    Mesh, Material,
};

pub struct Model {
    meshes:Rc<Vec<Mesh>>,
}

impl Model {
    pub fn new( meshes:Rc<Vec<Mesh>> ) -> Self {
        Self { meshes }
    }

    pub fn render( &self, material:&Material ) {
        material.use_material();
        for mesh in self.meshes.iter() {
            mesh.render();
        }
    }
}