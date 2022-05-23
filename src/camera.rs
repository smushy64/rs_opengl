use crate::transform::Transform;
use fmath::types::*;

pub struct Camera {
    pub transform: Transform,
    projection_mat: Matrix4x4,
    projection_mode: ProjectionMode,
}

impl Camera {

    pub fn new( projection_mode:ProjectionMode ) -> CameraBuilder {
        CameraBuilder::default( projection_mode )
    }

    pub fn view( &self ) -> Matrix4x4 {
        Matrix4x4::new_look_at(
            self.transform.get_position(),
            &( *self.transform.get_position() + self.transform.forward() ),
            &Vector3::new_up()
        )
    }

    pub fn projection( &self ) -> &Matrix4x4 {
        &self.projection_mat
    }

    pub fn get_projection_mode( &self ) -> &ProjectionMode {
        &self.projection_mode
    }

}

pub struct CameraBuilder {
    transform: Transform,

    fov: f32,
    aspect_ratio: f32,
    near: f32, far: f32,

    projection_mode: ProjectionMode
}

impl CameraBuilder {

    fn default( projection_mode:ProjectionMode ) -> Self {
        Self {
            transform: Transform::new(),
            fov: 1.5708, // 90.0
            aspect_ratio: 1.77777,
            near: 0.1, far: 100.0,
            projection_mode
        }
    }

    pub fn set_transform( mut self, transform:Transform ) -> Self {
        self.transform = transform;
        self
    }

    pub fn set_fov( mut self, fov_rad:f32 ) -> Self {
        self.fov = fov_rad;
        self
    }

    pub fn set_aspect_ratio( mut self, aspect_ratio:f32 ) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn set_clipping_fields( mut self, near:f32, far:f32 ) -> Self {
        self.near = near;
        self.far  = far;
        self
    }

    pub fn build( self ) -> Camera {

        let projection = match self.projection_mode {
            ProjectionMode::Perspective => {
                Matrix4x4::new_perspective_projection(
                    self.fov, self.aspect_ratio,
                    self.near, self.far
                )
            },
        };

        Camera {
            transform: self.transform,
            projection_mat: projection,
            projection_mode: self.projection_mode,
        }

    }

}

pub enum ProjectionMode {
    // Orthographic,
    Perspective,
}

impl core::fmt::Display for ProjectionMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let text = match self {
            ProjectionMode::Perspective => "Perspective",
        };

        write!( f, "{}", text )
    }
}