use fmath::types::*;
use core::fmt;

pub struct Camera {
    position: Vector3,
    rotation: Vector3,
    projection: Projection,
    resolution: ScreenResolution,
    near: f32, far: f32
}

impl Camera {
    pub fn new(
        position: Vector3, rotation: Vector3,
        projection: Projection, resolution: ScreenResolution,
        near: f32, far: f32
    ) -> Self {
        Self { position, rotation, projection, resolution, near, far }
    }

    pub fn position( &self ) -> &Vector3 { &self.position }
    pub fn rotation( &self ) -> &Vector3 { &self.rotation }

    pub fn position_mut( &mut self ) -> &mut Vector3 { &mut self.position }
    pub fn rotation_mut( &mut self ) -> &mut Vector3 { &mut self.rotation }

    pub fn translate( &mut self, delta:&Vector3 ) { self.position = self.position + *delta }
    pub fn rotate( &mut self, delta:&Vector3 )    { self.rotation = self.rotation + *delta }

    pub fn new_forward( &self ) -> Vector3 { Vector3::new_basis_forward( self.rotation[1], self.rotation[0] ) }

    pub fn new_view( &self, forward:Vector3 ) -> Matrix4x4 {
        Matrix4x4::new_look_at( &self.position, &(self.position + forward), &Vector3::new_up() )
    }

    pub fn orthographic_size( &self ) -> Option<f32> {
        match &self.projection {
            Projection::Orthographic(o) => Some( o.size() ),
            _ => None,
        }
    }

    pub fn fov( &self ) -> Option<f32> {
        match &self.projection {
            Projection::Perspective(p) => Some( p.fov() ),
            _ => None,
        }
    }

    pub fn new_projection( &self ) -> Matrix4x4 {
        match &self.projection {
            Projection::Orthographic(o) =>
                o.projection(
                    self.resolution.aspect_ratio(),
                    self.near, self.far
                ),
            Projection::Perspective(p) =>
                p.projection(
                    self.resolution.aspect_ratio(),
                    self.near, self.far
                ),
        }
    }

}

impl fmt::Display for Camera {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Camera {} {} | Near Clip: {} Far Clip: {}",
            self.projection, self.resolution, self.near, self.far )
    }
}

pub enum Projection {
    Orthographic(Orthographic),
    Perspective(Perspective),
}

impl Projection {
    pub fn new_ortho( size:f32 ) -> Self {
        Self::Orthographic( Orthographic::new( size ) )
    }

    pub fn ortho_default() -> Self {
        Self::Orthographic( Orthographic::default() )
    }

    pub fn new_perspective( fov_rad:f32 ) -> Self {
        Self::Perspective( Perspective::new( fov_rad ) )
    }

    pub fn perspective_default() -> Self {
        Self::Perspective( Perspective::default() )
    }
}

impl fmt::Display for Projection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Projection::Orthographic(o) => format!("{}",o),
            Projection::Perspective(p)   => format!("{}",p),
        };
        write!( f, "{}", msg )
    }
}

pub struct Orthographic {
    /// camera's half size
    size:f32
}

impl Orthographic {

    pub fn new( size:f32 ) -> Self { Self { size } }
    pub fn default() -> Self { Self::new( 2.0 ) }

    pub fn size( &self ) -> f32 { self.size }

    pub fn projection( &self, aspect_ratio:f32, near:f32, far:f32 ) -> Matrix4x4 {
        Matrix4x4::new_orthographic_projection(
            -(self.size * aspect_ratio), self.size * aspect_ratio,
            -self.size, self.size,
            near, far
        )
    }
}

impl fmt::Display for Orthographic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Orthographic | Size: {}", self.size() )
    }
}

pub struct Perspective {
    fov_rad: f32,
}

impl Perspective {

    pub fn new( fov_rad:f32 ) -> Self { Self { fov_rad } }
    pub fn default() -> Self { Self::new( 65f32.to_radians() ) }

    pub fn fov( &self ) -> f32 { self.fov_rad }

    pub fn projection( &self, aspect_ratio:f32, near:f32, far:f32 ) -> Matrix4x4 {
        Matrix4x4::new_perspective_projection(
            self.fov_rad, aspect_ratio, near, far
        )
    }

}

impl fmt::Display for Perspective {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Perspective | Field of View: {}", self.fov().to_degrees() )
    }
}

pub struct ScreenResolution {
    resolution:Vector2,
    aspect_ratio:f32
}

impl ScreenResolution {
    pub fn new( resolution:Vector2 ) -> Self {
        let aspect_ratio = resolution[0] / resolution[1];
        Self { resolution, aspect_ratio }
    }

    pub fn resolution(&self) -> &Vector2 { &self.resolution }
    pub fn aspect_ratio(&self) -> f32 { self.aspect_ratio }

    pub fn width(&self) -> f32 { self.resolution[0] }
    pub fn height(&self) -> f32 { self.resolution[1] }
}

impl fmt::Display for ScreenResolution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Resolution | Width: {} Height: {}", self.width(), self.height() )
    }
}
