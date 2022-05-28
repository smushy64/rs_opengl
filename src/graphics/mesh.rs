#[allow(unused_imports)]
use fmath::types::*;
use gl::types::*;

use super::Vertex;

#[derive(Debug, Clone)]
pub struct Mesh {

    pub vertices: Vec<Vertex>,
    pub indeces:  Vec<GLuint>,

    // opengl render data
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,

    index_count: GLsizei,

}

impl Mesh {

    pub fn new( vertices: Vec<Vertex>, indeces: Vec<GLuint> ) -> Self {

        const VERTEX_SIZE:GLint = 32;
        const U32_SIZE:GLint    =  4;
        const NORMAL_PTR_OFFSET:GLint = 12;
        const UV_PTR_OFFSET:GLint     = 24;

        let index_count = indeces.len() as GLint;

        let vertices_size = ( vertices.len() * VERTEX_SIZE as usize ) as GLsizeiptr;
        let indeces_size  = ( index_count * U32_SIZE ) as GLsizei;

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
                indeces_size as GLsizeiptr,
                indeces.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            // position
            gl::EnableVertexAttribArray( 0 );
            gl::VertexAttribPointer(
                0, 3, gl::FLOAT,
                gl::FALSE, VERTEX_SIZE,
                0 as *const GLvoid
            );
            // normals
            gl::EnableVertexAttribArray( 1 );
            gl::VertexAttribPointer(
                1, 3, gl::FLOAT,
                gl::FALSE, VERTEX_SIZE,
                NORMAL_PTR_OFFSET as *const GLvoid,
            );
            // uvs
            gl::EnableVertexAttribArray( 2 );
            gl::VertexAttribPointer(
                2, 2, gl::FLOAT,
                gl::FALSE, VERTEX_SIZE,
                UV_PTR_OFFSET as *const GLvoid,
            );

        }

        Self {
            vertices,
            indeces,
            vao, vbo, ebo,
            index_count,
        }
    }

    pub fn bind_buffers(&self) {
        unsafe {
            gl::BindVertexArray( self.vao );
            gl::BindBuffer( gl::ARRAY_BUFFER, self.vbo );
            gl::BindBuffer( gl::ELEMENT_ARRAY_BUFFER, self.ebo );
        }
    }

    pub fn render(&self) {
        unsafe {
            self.bind_buffers();
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                core::ptr::null_mut() as *const GLvoid
            );
        }
    }

    pub fn vertex_count( &self )   -> usize { self.index_count as usize }
    pub fn triangle_count( &self ) -> usize { ( self.index_count / 3 ) as usize }

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