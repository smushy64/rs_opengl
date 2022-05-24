use fmath::types::*;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: Vector3,
    pub normal:   Vector3,
    pub uv:       Vector2,
}

impl core::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "position: {} normal: {} uv: {}", self.position, self.normal, self.uv )
    }
}