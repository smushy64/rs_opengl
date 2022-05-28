#[allow(unused_imports)]
use fmath::types::*;
use gl::types::*;

use super::{ Vertex, Culling };

#[derive(Debug, Clone)]
pub struct Mesh {

    pub vertices: Vec<Vertex>,
    pub indeces:  Vec<GLuint>,
    pub culling: Culling,

    // opengl render data
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,

    index_count: GLsizei,

}

impl Mesh {

    pub fn new_with_culling( vertices: Vec<Vertex>, indeces: Vec<GLuint>, culling: Culling ) -> Self {
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
            culling,
            vao, vbo, ebo,
            index_count,
        }
    }

    pub fn new( vertices: Vec<Vertex>, indeces: Vec<GLuint> ) -> Self {
        Self::new_with_culling( vertices, indeces, Culling::initialize() )
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
            self.culling.update_gl();
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
            culling: Culling::initialize(),
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

    Mesh::new_with_culling( vertices, indeces, Culling::initialize_disabled() )

}

#[allow(dead_code)]
pub fn generate_cube() -> Mesh {
    let vertices:Vec<Vertex> = vec![
        // front
        // top left
        Vertex{ position:Vector3::new(-0.5, 0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,1.0)},
        // top right
        Vertex{ position:Vector3::new( 0.5, 0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,1.0)},
        // bottom left
        Vertex{ position:Vector3::new(-0.5,-0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,0.0)},
        // bottom right
        Vertex{ position:Vector3::new( 0.5,-0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,0.0)},

        // back
        // top left
        Vertex{ position:Vector3::new( 0.5, 0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(1.0,1.0)},
        // top right
        Vertex{ position:Vector3::new(-0.5, 0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(0.0,1.0)},
        // bottom left
        Vertex{ position:Vector3::new( 0.5,-0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(1.0,0.0)},
        // bottom right
        Vertex{ position:Vector3::new(-0.5,-0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(0.0,0.0)},

        // left
        // top left
        Vertex{ position:Vector3::new(-0.5, 0.5,-0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(0.0,1.0)},
        // top right
        Vertex{ position:Vector3::new(-0.5, 0.5, 0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(1.0,1.0)},
        // bottom left
        Vertex{ position:Vector3::new(-0.5,-0.5,-0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(0.0,0.0)},
        // bottom right
        Vertex{ position:Vector3::new(-0.5,-0.5, 0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(1.0,0.0)},

        // right
        // top left
        Vertex{ position:Vector3::new(0.5,  0.5,  0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(1.0,1.0)},
        // top right
        Vertex{ position:Vector3::new(0.5,  0.5, -0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(0.0,1.0)},
        // bottom left
        Vertex{ position:Vector3::new(0.5, -0.5,  0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(1.0,0.0)},
        // bottom right
        Vertex{ position:Vector3::new(0.5, -0.5, -0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(0.0,0.0)},

        // top
        Vertex{ position:Vector3::new( 0.5,  0.5,  0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(1.0,1.0)},
        Vertex{ position:Vector3::new(-0.5,  0.5,  0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(0.0,1.0)},
        Vertex{ position:Vector3::new( 0.5,  0.5, -0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(1.0,0.0)},
        Vertex{ position:Vector3::new(-0.5,  0.5, -0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(0.0,0.0)},

        // bottom
        Vertex{ position:Vector3::new(-0.5, -0.5,  0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(0.0,1.0)},
        Vertex{ position:Vector3::new( 0.5, -0.5,  0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(1.0,1.0)},
        Vertex{ position:Vector3::new(-0.5, -0.5, -0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(0.0,0.0)},
        Vertex{ position:Vector3::new( 0.5, -0.5, -0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(1.0,0.0)},
    ];

    let indeces:Vec<u32> = vec![
        0, 2, 1,
        1, 2, 3,

        4, 6, 5,
        5, 6, 7,

        8, 10,  9,
        9, 10, 11,

        12, 14, 13,
        13, 14, 15,

        16, 18, 17,
        17, 18, 19,

        20, 22, 21,
        21, 22, 23,
    ];

    Mesh::new( vertices, indeces )
}