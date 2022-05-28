use fmath::types::*;
use core::fmt;

#[derive(Debug, Clone)]
pub struct Transform {
    position: Vector3,
    rotation: Vector3,
    size:     Vector3,

    transform: Matrix4x4,
    normal:    Matrix3x3,

    forward: Vector3,
    right:   Vector3,
    up:      Vector3,

    use_world_up: bool,
}

impl Transform {

    pub fn new( position:Vector3, rotation:Vector3, size:Vector3 ) -> Self {
        let transform = Matrix4x4::new_trs(
            position.as_array(), rotation.as_array(), size.as_array()
        );

        let normal = {
            match transform.inverse() {
                Some(inv) => Matrix4x4::transpose( inv ).as_matrix3x3(),
                None => Matrix3x3::new_zero(),
            }
        };

        let forward = Self::calc_forward_basis( rotation[1], rotation[0] );
        let mut up  = Vector3::new_up();
        let right   = Self::calc_right_basis( &forward, &up );
        up                   = Self::calc_up_basis( &forward, &right );

        Self {
            position,
            rotation,
            size,

            transform,
            normal,

            forward,
            right,
            up,

            use_world_up: false,
        }
    }
    pub fn new_zero() -> Self {
        Self::new( Vector3::new_zero(), Vector3::new_zero(), Vector3::new_one() )
    }
    pub fn new_with_position( position:Vector3 ) -> Self {
        Self::new( position, Vector3::new_zero(), Vector3::new_one() )
    }
    pub fn new_with_rotation( rotation:Vector3 ) -> Self {
        Self::new( Vector3::new_zero(), rotation, Vector3::new_one() )
    }
    pub fn new_with_size( size:Vector3 ) -> Self {
        Self::new( Vector3::new_zero(), Vector3::new_zero(), size )
    }

    pub fn use_world_up( &mut self )      { self.use_world_up = true }
    pub fn dont_use_world_up( &mut self ) { self.use_world_up = false }
    pub fn is_using_world_up( &self ) -> bool { self.use_world_up }

    pub fn position( &self ) -> &Vector3 { &self.position }
    pub fn rotation( &self ) -> &Vector3 { &self.rotation }
    pub fn size    ( &self ) -> &Vector3 { &self.size }

    pub fn position_mut( &mut self ) -> &mut Vector3 { &mut self.position }
    pub fn rotation_mut( &mut self ) -> &mut Vector3 { &mut self.rotation }
    pub fn size_mut    ( &mut self ) -> &mut Vector3 { &mut self.size }

    pub fn translate( &mut self, delta:&Vector3 ) { self.position = self.position + *delta; }
    pub fn rotate   ( &mut self, delta:&Vector3 ) { self.rotation = self.rotation + *delta; }
    pub fn scale    ( &mut self, delta:&Vector3 ) { self.size = Vector3::scale( &self.size , &delta ); }

    pub fn forward( &self ) -> &Vector3 { &self.forward }
    pub fn right( &self )   -> &Vector3 { &self.right   }
    pub fn up( &self )      -> &Vector3 { &self.up      }

    pub fn current_forward( &mut self ) -> &Vector3 { self.update_basis_vectors(); &self.forward }
    pub fn current_right( &mut self )   -> &Vector3 { self.update_basis_vectors(); &self.right   }
    pub fn current_up( &mut self )      -> &Vector3 { self.update_basis_vectors(); &self.up      }

    pub fn update_transform_matrix(&mut self) {
        self.transform = Matrix4x4::new_trs(
            self.position().as_array(),
            self.rotation().as_array(),
            self.size().as_array()
        );
    }

    pub fn update_normal_matrix(&mut self) {
        self.normal = match self.transform.inverse() {
            Some(inv) => Matrix4x4::transpose( inv ).as_matrix3x3(),
            None => return,
        };
    }

    pub fn update_basis_vectors(&mut self) {
        self.forward = Self::calc_forward_basis( self.rotation[1], self.rotation[0] );
        if self.use_world_up {
            self.up    = Vector3::new_up();
            self.right = Self::calc_right_basis( &self.forward, &self.up );
        } else {
            self.right = Self::calc_right_basis( &self.forward, &self.up );
            self.up    = Self::calc_up_basis( &self.forward, &self.right );
        }
    }

    pub fn transform_matrix( &self ) -> &Matrix4x4 { &self.transform }
    pub fn normal_matrix   ( &self ) -> &Matrix3x3 { &self.normal }

    pub fn current_transform_matrix( &mut self ) -> &Matrix4x4 {
        self.update_transform_matrix();
        &self.transform
    }
    pub fn current_normal_matrix ( &mut self ) -> &Matrix3x3 {
        self.update_transform_matrix();
        self.update_normal_matrix();
        &self.normal
    }

    fn calc_forward_basis( yaw:f32, pitch:f32 ) -> Vector3 {
        Vector3::new(
            yaw.cos() * pitch.cos(),
            pitch.sin(),
            yaw.sin() * pitch.cos(),
        ).normal()
    }

    fn calc_right_basis( forward:&Vector3, up:&Vector3 ) -> Vector3 {
        Vector3::cross( forward, up ).normal()
    }

    fn calc_up_basis( forward:&Vector3, right:&Vector3 ) -> Vector3 {
        Vector3::cross( forward, right )
    }

}

impl fmt::Display for Transform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f,
            "Position: {} Rotation (radians): {} Scale: {}",
            self.position(), self.rotation(), self.size()
        )
    }
}