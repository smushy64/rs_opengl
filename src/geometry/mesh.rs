#[allow(unused_imports)]
use fmath::types::*;
use gl::types::*;

use super::Vertex;

#[derive(Clone)]
pub struct Mesh {

    pub vertices: Vec<Vertex>,
    pub indeces:  Vec<GLuint>,

    // opengl render data
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,

    index_count: GLint,

}

impl Mesh {

    pub fn new( vertices: Vec<Vertex>, indeces: Vec<GLuint> ) -> Self {

        use core::mem::size_of;

        let index_count = indeces.len() as GLint;

        let vertices_size = ( vertices.len() * size_of::<Vertex>() ) as GLsizeiptr;
        let indeces_size  = ( index_count as usize * size_of::<GLuint>() ) as GLsizeiptr;

        let f32_size = size_of::<f32>();
        let normal_ptr_offset = ( 3 * f32_size ) as *const GLvoid;
        let uv_ptr_offset     = ( 6 * f32_size ) as *const GLvoid;

        // position = 3, normal = 3, uv = 2
        let stride = 8 * f32_size as GLsizei;

        let mut vbo:GLuint = 0;
        let mut vao:GLuint = 0;
        let mut ebo:GLuint = 0;

        unsafe {
            // vertex array object
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
    
            // vertex buffer object
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer( gl::ARRAY_BUFFER, vbo );
            gl::BufferData(
                gl::ARRAY_BUFFER,
                vertices_size,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );
    
            // element buffer object
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                indeces_size,
                indeces.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            // position
            gl::EnableVertexAttribArray( 0 );
            gl::VertexAttribPointer(
                0, 3, gl::FLOAT,
                gl::FALSE, stride,
                0 as *const GLvoid
            );
            // normals
            gl::EnableVertexAttribArray( 1 );
            gl::VertexAttribPointer(
                1, 3, gl::FLOAT,
                gl::FALSE, stride,
                normal_ptr_offset,
            );
            // uvs
            gl::EnableVertexAttribArray( 2 );
            gl::VertexAttribPointer(
                2, 2, gl::FLOAT,
                gl::FALSE, stride,
                uv_ptr_offset,
            );

            // unbind vao
            gl::BindVertexArray( 0 );

        }

        Self {
            vertices,
            indeces,
            vao, vbo, ebo,
            index_count,
        }
    }

    pub fn render(&self) {
        unsafe {
            gl::BindVertexArray( self.vao );
            gl::BindBuffer( gl::ARRAY_BUFFER, self.vbo );
            gl::BindBuffer( gl::ELEMENT_ARRAY_BUFFER, self.ebo );
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                core::ptr::null_mut() as *const GLvoid
            );
            gl::BindVertexArray( 0 );
            gl::BindBuffer( gl::ARRAY_BUFFER, 0 );
            gl::BindBuffer( gl::ELEMENT_ARRAY_BUFFER, 0 );
        }
    }

    pub fn vertex_count( &self ) -> GLint {
        self.index_count
    }

    pub fn triangle_count( &self ) -> GLint {
        self.index_count / 3
    }

    pub fn get_mesh_data( &self ) -> String {
        format!( "vertex count: {} triangle count: {}",
            self.vertex_count(),
            self.triangle_count()
        )
    }

    pub unsafe fn empty() -> Self {
        Self {
            vertices: Vec::new(),
            indeces:  Vec::new(),
            vao: 0, vbo: 0, ebo: 0,
            index_count: 0
        }
    }

}

impl Drop for Mesh {
    fn drop(&mut self) {
        let vertex_arrays = [ self.vao ];
        let buffers = [ self.vbo, self.ebo ];
        unsafe {
            gl::DeleteVertexArrays(
                vertex_arrays.len() as GLsizei,
                vertex_arrays.as_ptr() as *const GLuint
            );
            gl::DeleteBuffers(
                buffers.len() as GLsizei,
                buffers.as_ptr() as *const GLuint
            );
        }
    }
}

#[allow(dead_code)]
pub fn generate_plane() -> Mesh {

    let vertices:Vec<Vertex> = vec![
        Vertex{ position:Vector3::new(-0.5, 0.5,0.0), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,1.0) },
        Vertex{ position:Vector3::new(-0.5,-0.5,0.0), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,0.0) },
        Vertex{ position:Vector3::new( 0.5, 0.5,0.0), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,1.0) },
        Vertex{ position:Vector3::new( 0.5,-0.5,0.0), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,0.0) },
    ];

    let indeces:Vec<GLuint> = vec![
        0, 1, 2,
        2, 1, 3,
    ];

    Mesh::new( vertices, indeces )

}

#[allow(dead_code)]
pub fn generate_cone() -> Mesh {

    let vertices:Vec<Vertex> = vec![
        Vertex{ position:Vector3::new( 0.0, 0.0,0.0), normal:Vector3::new( 0.00, 0.00,-1.0), uv:Vector2::new(0.0, 0.0) },
        Vertex{ position:Vector3::new( 0.0, 1.0,1.0), normal:Vector3::new( 0.00, 1.00, 0.0), uv:Vector2::new(0.0, 0.0) },
        Vertex{ position:Vector3::new( 0.0,-1.0,1.0), normal:Vector3::new( 0.00,-1.00, 0.0), uv:Vector2::new(0.0, 0.0) },
        Vertex{ position:Vector3::new(-0.8, 0.5,1.0), normal:Vector3::new(-0.77, 0.77, 0.0), uv:Vector2::new(0.0, 0.0) },
        Vertex{ position:Vector3::new(-0.8,-0.5,1.0), normal:Vector3::new(-0.77,-0.77, 0.0), uv:Vector2::new(0.0, 0.0) },
        Vertex{ position:Vector3::new( 0.8, 0.5,1.0), normal:Vector3::new( 0.77, 0.77, 0.0), uv:Vector2::new(0.0, 0.0) },
        Vertex{ position:Vector3::new( 0.8,-0.5,1.0), normal:Vector3::new( 0.77,-0.77, 0.0), uv:Vector2::new(0.0, 0.0) },
    ];

    let indeces:Vec<u32> = vec![
        // top right
        0, 1, 5,
        // mid right
        0, 5, 6,
        // bottom right
        0, 6, 2,
        // bottom left
        0, 4, 2,
        // mid left
        0, 3, 4,
        // top left
        0, 1, 3,
        // cap right top
        5, 1, 6,
        // cap right bottom
        6, 1, 2,
        // cap left bottom
        2, 3, 4,
        // cap left top
        2, 1, 3,
    ];

    Mesh::new( vertices, indeces )
}

#[allow(dead_code)]
pub fn generate_cube() -> Mesh {
    let vertices:Vec<Vertex> = vec![
        // front
        Vertex{ position:Vector3::new(-0.5, 0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,1.0)},
        Vertex{ position:Vector3::new( 0.5, 0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,1.0)},
        Vertex{ position:Vector3::new(-0.5,-0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,0.0)},
        Vertex{ position:Vector3::new( 0.5,-0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,0.0)},

        // back
        Vertex{ position:Vector3::new(-0.5, 0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(0.0,1.0)},
        Vertex{ position:Vector3::new( 0.5, 0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(1.0,1.0)},
        Vertex{ position:Vector3::new(-0.5,-0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(0.0,0.0)},
        Vertex{ position:Vector3::new( 0.5,-0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(1.0,0.0)},

        // left
        Vertex{ position:Vector3::new(-0.5, 0.5,-0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(0.0,1.0)},
        Vertex{ position:Vector3::new(-0.5, 0.5, 0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(1.0,1.0)},
        Vertex{ position:Vector3::new(-0.5,-0.5,-0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(0.0,0.0)},
        Vertex{ position:Vector3::new(-0.5,-0.5, 0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(1.0,0.0)},

        // right
        Vertex{ position:Vector3::new(0.5,  0.5, -0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(0.0,1.0)},
        Vertex{ position:Vector3::new(0.5,  0.5,  0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(1.0,1.0)},
        Vertex{ position:Vector3::new(0.5, -0.5, -0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(0.0,0.0)},
        Vertex{ position:Vector3::new(0.5, -0.5,  0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(1.0,0.0)},

        // top
        Vertex{ position:Vector3::new(-0.5,  0.5,  0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(0.0,1.0)},
        Vertex{ position:Vector3::new( 0.5,  0.5,  0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(1.0,1.0)},
        Vertex{ position:Vector3::new(-0.5,  0.5, -0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(0.0,0.0)},
        Vertex{ position:Vector3::new( 0.5,  0.5, -0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(1.0,0.0)},

        // bottom
        Vertex{ position:Vector3::new(-0.5, -0.5,  0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(0.0,1.0)},
        Vertex{ position:Vector3::new( 0.5, -0.5,  0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(1.0,1.0)},
        Vertex{ position:Vector3::new(-0.5, -0.5, -0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(0.0,0.0)},
        Vertex{ position:Vector3::new( 0.5, -0.5, -0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(1.0,0.0)},
    ];

    let indeces:Vec<u32> = vec![
        0, 1, 2,
        1, 3, 2,

        4, 5, 6,
        5, 7, 6,

        8,  9, 10,
        9, 11, 10,

        12, 13, 14,
        13, 15, 14,

        16, 17, 18,
        17, 19, 18,

        20, 21, 22,
        21, 23, 22,
    ];

    Mesh::new( vertices, indeces )
}