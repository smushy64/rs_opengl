use fmath::types::*;

pub struct Transform {
    position: Vector3,
    rotation: Vector3,
    size:     Vector3,

    mat: Matrix4x4,
}

impl Transform {

    pub fn new() -> Self {
        Self {
            position: Vector3::new_zero(),
            rotation: Vector3::new_zero(),
            size:     Vector3::new_one(),

            mat: Matrix4x4::new_identity()
        }
    }

    pub fn forward( &self ) -> Vector3 {
        Vector3::new(
            self.yaw().cos() * self.pitch().cos(),
            self.pitch().sin(),
            self.yaw().sin() * self.pitch().cos(),
        ).normal()
    }

    pub fn calculate_right( forward:&Vector3, up:&Vector3 ) -> Vector3 {
        Vector3::cross( forward, up ).normal()
    }

    pub fn get_position( &self ) -> &Vector3 {
        &self.position
    }

    pub fn get_rotation( &self ) -> &Vector3 {
        &self.rotation
    }

    pub fn get_scale( &self ) -> &Vector3 {
        &self.size
    }

    pub fn set_position( &mut self, new_pos:Vector3 ) {
        self.position = new_pos;
        self.reset_mat();
    }

    pub fn set_rotation( &mut self, new_rot_euler:Vector3 ) {
        self.rotation = new_rot_euler;
        self.reset_mat();
    }

    pub fn set_scale( &mut self, new_scale:Vector3 ) {
        self.size = new_scale;
        self.reset_mat();
    }

    pub fn translate( &mut self, delta:&Vector3 ) {
        self.position = self.position + *delta;
        self.reset_mat();
    }

    pub fn rotate( &mut self, delta:&Vector3 ) {
        self.rotation = self.rotation + *delta;
        self.reset_mat();
    }
    
    pub fn roll( &self ) -> &f32 {
        &self.rotation[2]
    }

    pub fn yaw( &self ) -> &f32 {
        &self.rotation[1]
    }

    pub fn pitch( &self ) -> &f32 {
        &self.rotation[0]
    }
    
    pub fn roll_mut( &mut self ) -> &mut f32 {
        &mut self.rotation[2]
    }
    
    pub fn yaw_mut( &mut self ) -> &mut f32 {
        &mut self.rotation[1]
    }

    pub fn pitch_mut( &mut self ) -> &mut f32 {
        &mut self.rotation[0]
    }

    pub fn scale( &mut self, delta:&Vector3 ) {
        self.size = self.size + *delta;
        self.reset_mat();
    }

    pub fn set_transform( &mut self, position:Vector3, rotation:Vector3, scale:Vector3 ) {
        self.position = position;
        self.rotation = rotation;
        self.size = scale;
        self.reset_mat();
    }

    fn reset_mat(&mut self) {
        self.mat = Matrix4x4::new_trs(
            self.get_position().as_array(),
            self.get_rotation().as_array(),
            self.get_scale().as_array()
        );
    }

    pub fn transform_mat( &self ) -> &Matrix4x4 {
        &self.mat
    }

}

impl core::fmt::Display for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f,
            "Position: {} Rotation (radians): {} Scale: {}",
            self.get_position(), self.get_rotation(), self.get_scale()
        )
    }
}