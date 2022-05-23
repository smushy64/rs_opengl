use fmath::types::{ *, color::RGB };
use crate::{
    transform::Transform,
};

pub struct DirectionalLight {

    pub direction: Vector3,

    pub diffuse:   RGB,
    pub specular:  RGB,
    pub ambient:   RGB,

}

pub struct PointLight {

    pub position: Vector3,

    pub diffuse:   RGB,
    pub specular:  RGB,
    pub ambient:   RGB,

    pub constant:  f32,
    pub linear:    f32,
    pub quadratic: f32,

}

impl PointLight {

    pub fn transform( &self ) -> Transform {
        let mut transform = Transform::new();

        transform.set_position( self.position.clone() );
        transform.set_scale( Vector3::new_one() * 0.2 );

        transform
    }

}

pub struct SpotLight {

    pub position:  Vector3,
    pub direction: Vector3,

    pub inner_cutoff: f32,
    pub outer_cutoff: f32,

    pub constant:  f32,
    pub linear:    f32,
    pub quadratic: f32,

    pub diffuse:   RGB,
    pub specular:  RGB,
    pub ambient:   RGB,

}