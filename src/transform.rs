use fmath::types::*;

pub struct Transform {
    position:Vector3,
    rotation_euler:Vector3,
    scale:Vector3,
    mat:Matrix4x4
}

impl Transform {

    pub fn new() -> Self {
        Self {
            position:Vector3::new_zero(),
            rotation_euler:Vector3::new_zero(),
            scale:Vector3::new_one(),
            mat:Matrix4x4::new_identity()
        }
    }

    pub fn position( &self ) -> &Vector3 {
        &self.position
    }

    pub fn rotation( &self ) -> &Vector3 {
        &self.rotation_euler
    }

    pub fn scale( &self ) -> &Vector3 {
        &self.scale
    }

    pub fn set_position( &mut self, new_pos:Vector3 ) {
        self.position = new_pos;
        self.reset_mat();
    }

    pub fn set_rotation( &mut self, new_rot_euler:Vector3 ) {
        self.rotation_euler = new_rot_euler;
        self.reset_mat();
    }

    pub fn set_scale( &mut self, new_scale:Vector3 ) {
        self.scale = new_scale;
        self.reset_mat();
    }

    pub fn set_trs( &mut self, position:Vector3, rotation:Vector3, scale:Vector3 ) {
        self.position = position;
        self.rotation_euler = rotation;
        self.scale = scale;
        self.reset_mat();
    }

    pub fn set_mat( &mut self, mat:Matrix4x4 ) {
        self.mat = mat;
    }

    fn reset_mat(&mut self) {
        self.mat =
            Matrix4x4::new_trs( self.position().as_array(), self.rotation().as_array(), self.scale().as_array() );
    }

    pub fn mat_raw( &self ) -> &[f32; 16] {
        self.mat.as_array()
    }

    pub fn mat_ptr( &self ) -> *const f32 {
        self.mat.as_array().as_ptr()
    }

    pub fn mat( &self ) -> &Matrix4x4 {
        &self.mat
    }

}

use core::fmt::Display;

impl Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f,
            "Position: {} Rotation(euler): {} Scale: {}",
            self.position(), self.rotation(), self.scale()
        )
    }
}