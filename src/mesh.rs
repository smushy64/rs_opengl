use gl::types::*;

use core::mem::size_of;

pub struct Mesh {
    vbo:GLuint,
    vao:GLuint,
    ebo:GLuint,
    index_count:GLint
    // material:Material
}

impl Mesh {

    pub fn render( &self ) {
        unsafe {
            gl::BindVertexArray( self.vao );
            gl::BindBuffer( gl::ARRAY_BUFFER, self.ebo );

            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count,
                gl::UNSIGNED_INT,
                core::ptr::null_mut() as *const GLvoid
            );
        }
    }

    pub fn get_vbo(&self) -> GLuint {
        self.vbo
    }

    pub fn get_vao(&self) -> GLuint {
        self.vao
    }

    pub fn get_ebo(&self) -> GLuint {
        self.ebo
    }

    pub fn new( vertices:Vec<f32> ) -> MeshBuilder {
        MeshBuilder {
            vertices,
            indeces: Vec::new(),
            data_order: Vec::new(),
        }
    }

    pub fn generate(
        vertices:Vec<f32>,
        indeces:Vec<u32>,
        data_order:Vec<MeshData>,
    ) -> Self {

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
                ( vertices.len() * size_of::<f32>() ) as GLsizeiptr,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );
    
            // element buffer object
            gl::GenBuffers(1, &mut ebo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                ( indeces.len() * size_of::<u32>() ) as GLsizeiptr,
                indeces.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );
        }

        Self::generate_vertex_attribute_arrays( data_order );

        Mesh {
            vbo, vao, ebo,
            index_count:indeces.len() as GLint
        }

    }

    fn generate_vertex_attribute_arrays( data_order:Vec<MeshData> ) {

        let mut stride:usize = 0;
        let mut sizes:Vec<GLint> = Vec::with_capacity(data_order.len());
        for data in data_order.iter() {
            let size = match data {
                MeshData::UV => 2,
                _ => 3,
            };
            sizes.push(size);
            stride += size as usize;
        }

        stride = stride * size_of::<f32>();

        let mut i = 0;
        let mut pointer_offset = 0;
        while i < data_order.len() {

            if i != 0 {
                pointer_offset += sizes[i - 1] as usize * size_of::<f32>();
            }

            unsafe {
                gl::EnableVertexAttribArray(i as GLuint);

                gl::VertexAttribPointer(
                    i as GLuint, sizes[i],
                    gl::FLOAT, gl::FALSE,
                    stride as GLsizei,
                    pointer_offset as *const GLvoid
                );

            }

            i += 1;
        }
    
    }

}

pub struct MeshBuilder {
    vertices:Vec<f32>,
    indeces:Vec<u32>,
    data_order:Vec<MeshData>,
}

impl MeshBuilder {

    pub fn set_data_order( mut self, data_order:Vec<MeshData> ) -> Self {
        self.data_order = data_order;
        self
    }

    pub fn set_indeces( mut self, indeces:Vec<u32> ) -> Self {
        self.indeces = indeces;
        self
    }

    pub fn set_vertices( mut self, vertices:Vec<f32> ) -> Self {
        self.vertices = vertices;
        self
    }

    pub fn build( self ) -> Mesh {
        Mesh::generate(self.vertices, self.indeces, self.data_order)
    }

}

pub enum MeshData {
    Position,
    Color,
    Normal,
    UV,
}

pub fn generate_cube() -> Mesh {
    let vertices:Vec<f32> = vec![
        // front
        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */  0.0, 0.0,  1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */  0.0, 0.0,  1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */  0.0, 0.0,  1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */  0.0, 0.0,  1.0, /* UVs */  1.0,  0.0,

        // back
        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */  0.0, 0.0, -1.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5, -0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */  0.0, 0.0, -1.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */  0.0, 0.0, -1.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */  0.0, 0.0, -1.0, /* UVs */  1.0,  0.0,

        // left
        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */ -1.0, 0.0,  0.0, /* UVs */  0.0,  1.0,
        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */ -1.0, 0.0,  0.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */ -1.0, 0.0,  0.0, /* UVs */  0.0,  0.0,
        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */ -1.0, 0.0,  0.0, /* UVs */  1.0,  0.0,

        // right
        /* Positions */  0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */  1.0, 0.0,  0.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */  1.0, 0.0,  0.0, /* UVs */  1.0,  1.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */  1.0, 0.0,  0.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */  1.0, 0.0,  0.0, /* UVs */  1.0,  0.0,

        // top
        /* Positions */ -0.5,  0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */  0.0, 1.0,  0.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5,  0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */  0.0, 1.0,  0.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5,  0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */  0.0, 1.0,  0.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5,  0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */  0.0, 1.0,  0.0, /* UVs */  1.0,  0.0,

        // bottom
        /* Positions */ -0.5, -0.5,  0.5, /* Color */ 1.0, 0.0, 0.0, /* Normals */   0.0, -1.0, 0.0, /* UVs */  0.0,  1.0,
        /* Positions */  0.5, -0.5,  0.5, /* Color */ 0.0, 1.0, 0.0, /* Normals */   0.0, -1.0, 0.0, /* UVs */  1.0,  1.0,
        /* Positions */ -0.5, -0.5, -0.5, /* Color */ 0.0, 0.0, 1.0, /* Normals */   0.0, -1.0, 0.0, /* UVs */  0.0,  0.0,
        /* Positions */  0.5, -0.5, -0.5, /* Color */ 1.0, 0.0, 1.0, /* Normals */   0.0, -1.0, 0.0, /* UVs */  1.0,  0.0,
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

    Mesh::new( vertices )
        .set_indeces( indeces )
        .set_data_order(vec![
            MeshData::Position,
            MeshData::Color,
            MeshData::Normal,
            MeshData::UV,])
        .build()
}

// let mut vbo:GLuint = 0;
// let mut vao:GLuint = 0;
// let mut ebo:GLuint = 0;

// // load cube into gl
// unsafe {
//     use core::mem::size_of;

//     // vertex array object
//     gl::GenVertexArrays(1, &mut vao);
//     gl::BindVertexArray(vao);

//     // vertex buffer object
//     gl::GenBuffers(1, &mut vbo);
//     gl::BindBuffer( gl::ARRAY_BUFFER, vbo );
//     gl::BufferData(
//         gl::ARRAY_BUFFER,
//         ( vertices.len() * size_of::<f32>() ) as GLsizeiptr,
//         vertices.as_ptr() as *const GLvoid,
//         gl::STATIC_DRAW
//     );

//     // ebo
//     gl::GenBuffers(1, &mut ebo);
//     gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
//     gl::BufferData(
//         gl::ELEMENT_ARRAY_BUFFER,
//         ( indeces.len() * size_of::<u32>() ) as GLsizeiptr,
//         indeces.as_ptr() as *const GLvoid,
//         gl::STATIC_DRAW
//     );

//     // vertex attrib pointer
//     let stride = 11;
//     // vertices
//     gl::EnableVertexAttribArray(0);
//     gl::VertexAttribPointer(
//         0, 3,
//         gl::FLOAT, gl::FALSE,
//         ( stride * size_of::<f32>() ) as GLsizei,
//         0 as *const GLvoid
//     );
//     // vert colors
//     gl::EnableVertexAttribArray(1);
//     gl::VertexAttribPointer(
//         1, 3,
//         gl::FLOAT, gl::FALSE,
//         ( stride * size_of::<f32>() ) as GLsizei,
//         ( 3 * size_of::<f32>() ) as *const GLvoid
//     );
//     // normals
//     gl::EnableVertexAttribArray(2);
//     gl::VertexAttribPointer(
//         2, 3,
//         gl::FLOAT, gl::FALSE,
//         ( stride * size_of::<f32>() ) as GLsizei,
//         ( 6 * size_of::<f32>() ) as *const GLvoid
//     );
//     // texcoords
//     gl::EnableVertexAttribArray(3);
//     gl::VertexAttribPointer(
//         3, 2,
//         gl::FLOAT, gl::FALSE,
//         ( stride * size_of::<f32>() ) as GLsizei,
//         ( 9 * size_of::<f32>() ) as *const GLvoid
//     );

//     gl::BindVertexArray( 0 );
//     gl::BindBuffer( gl::ARRAY_BUFFER, 0 );
// }

// gl::BindVertexArray( vao );
// gl::BindBuffer( gl::ARRAY_BUFFER, ebo );

// // draw main cube ===========================================

// gl::DrawElements(
//     gl::TRIANGLES,
//     indeces.len() as GLint,
//     gl::UNSIGNED_INT,
//     core::ptr::null_mut() as *const GLvoid
// );