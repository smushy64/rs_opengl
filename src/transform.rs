use fmath::types::*;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Transform {
    position: Vector3,
    rotation: Quaternion,
    scale:    Vector3,
}

impl Transform {

    pub fn new( position: Vector3, rotation: Quaternion, scale: Vector3 ) -> Self {
        Self { position, rotation, scale }
    }

    pub fn new_from_euler( position: Vector3, euler: Vector3, scale: Vector3 ) -> Self {
        Self { position, rotation: Quaternion::from_euler_angles(euler), scale }
    }

    pub fn new_default() -> Self {
        Self {
            position: Vector3::new_zero(),
            rotation: Quaternion::new_identity(),
            scale: Vector3::new_one()
        }
    }

    pub fn position(&self) -> &Vector3 { &self.position }
    pub fn rotation(&self) -> &Quaternion { &self.rotation }
    pub fn scale(&self)    -> &Vector3 { &self.scale }

    pub fn position_mut(&mut self) -> &mut Vector3 { &mut self.position }
    pub fn rotation_mut(&mut self) -> &mut Quaternion { &mut self.rotation }
    pub fn scale_mut   (&mut self) -> &mut Vector3 { &mut self.scale }

    pub fn rotation_as_euler(&self) -> Vector3 { self.rotation.as_euler_angles() }
    
    pub fn translate( &mut self, delta:Vector3 ) {
        self.position = self.position + delta;
    }
    
    pub fn rotate( &mut self, delta:Quaternion ) {
        self.rotation = delta * self.rotation;
    }
    pub fn set_rotation(&mut self, new_rot:Quaternion) {
        self.rotation = new_rot;
    }
    pub fn set_rotation_from_euler(&mut self, new_rot:Vector3) {
        self.rotation = Quaternion::from_euler_angles( new_rot );
    }

    pub fn uniform_scale( &mut self, delta:f32 ) {
        self.scale = self.scale * delta;
    }

    pub fn component_wise_scale( &mut self, delta:Vector3 ) {
        self.scale = Vector3::scale(self.scale(), &delta);
    }

    pub fn new_up(&self)      -> Vector3 { self.rotation * Vector3::new_up() }
    pub fn new_right(&self)   -> Vector3 { self.rotation * Vector3::new_right() }
    pub fn new_forward(&self) -> Vector3 { self.rotation * Vector3::new_forward() }

    pub fn new_basis(&self) -> BasisVectors {
        BasisVectors {
            up:      self.new_up(),
            forward: self.new_forward(),
            right:   self.new_right(),
        }
    }

    pub fn as_matrix( &self ) -> Matrix4x4 {
        Matrix4x4::new_trs( self.position(), self.rotation(), self.scale() )
    }

}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f,
            "Position: {} Rotation: {} Scale: {}",
            self.position, self.rotation, self.scale
        )
    }
}

pub struct BasisVectors {
    pub up:      Vector3,
    pub forward: Vector3,
    pub right:   Vector3,
}
