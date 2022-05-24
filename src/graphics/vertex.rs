use fmath::types::*;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Vertex {
    pub position: Vector3,
    pub normal:   Vector3,
    pub uv:       Vector2,
}

impl Vertex {

    pub fn empty() -> Self {
        Self { position: Vector3::new_zero(), normal: Vector3::new_zero(), uv: Vector2::new_zero(), }
    }

    pub fn clear(&mut self) { self.position.clear(); self.normal.clear(); self.uv.clear(); }

    pub unsafe fn from_vec_unchecked( vertices:Vec<f32> ) -> Vec<Vertex> {
        let mut buffer = Vec::new();
        let mut v_buffer = Vertex::empty();
        // position position position normal normal normal uv uv
        let mut counter = 0;
        for v in vertices {
            if      counter < 3 { v_buffer.position[counter]   = v; }
            else if counter < 6 { v_buffer.normal[counter - 3] = v; }
            else if counter < 8 { v_buffer.uv[counter - 6]     = v; }

            counter += 1;
            if counter == 8 {
                counter = 0;
                buffer.push( v_buffer.clone() );
                v_buffer.clear();
            }
        }
        buffer
    }
}

impl core::fmt::Display for Vertex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "position: {} normal: {} uv: {}", self.position, self.normal, self.uv )
    }
}