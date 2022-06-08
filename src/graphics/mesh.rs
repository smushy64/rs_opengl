use std::mem::size_of;

#[allow(unused_imports)]
use fmath::types::*;
use gl::types::*;
use wavefront_obj::MeshOBJ;
use rs_gltf::glTF;
use crate::{ Rc, debugging::Error };

use super::Culling;

use core::fmt;

#[derive(Debug, Clone)]
pub struct Mesh {

    vertices: Vec<f32>,
    indices:  Vec<GLuint>,
    culling:  Culling,

    // opengl render data
    vao: GLuint,
    vbo: GLuint,
    ebo: GLuint,

    index_count: usize,

}

impl Mesh {

    pub fn from_obj( mesh_objs:Vec<MeshOBJ> ) -> Vec<Rc<Self>> {
        let mut mesh_buffer = Vec::with_capacity( mesh_objs.len() );
        for obj in mesh_objs.iter() {
            let (vert, idx) = obj.as_opengl_format();
            mesh_buffer.push( Self::from_raw(
                vert, idx,
                Culling::initialize(),
                vec![ MeshData::Position(0), MeshData::Normal(0), MeshData::UV(0) ],
                true
            ) )
        }
        mesh_buffer
    }

    /// WORK IN PROGRESS: material and texture data is currently being ignored :(
    pub fn from_gltf( gltf:glTF ) -> Result< Vec<Rc<Self>>, Error > {

        let meshes = &gltf.meshes
            .ok_or( Error::GLTFJsonError( "glTF does not contain any meshes!".to_owned() ) )?;

        let buffer_bytes = {
            let buffers = &gltf.buffers
                .ok_or( Error::GLTFJsonError( "glTF does not contain any buffers!".to_owned() ) )?;

            let mut bytes_buffer:Vec<Vec<u8>> = Vec::new();

            for buffer in buffers.iter() {
                let uri = buffer.uri()
                    .ok_or(Error::GLTFJsonError( "glTF buffer does not contain a URI!".to_owned() ))?;
                bytes_buffer.push(
                    uri.as_data()
                        .ok_or( Error::GLTFJsonError( "glTF uri does not contain any data!".to_owned() ) )?
                );
            }
            bytes_buffer
        };

        let buffer_views = &gltf.bufferViews
            .ok_or( Error::GLTFJsonError( "glTF does not contain any buffer views!".to_owned() ) )?;
        
        let accessors = &gltf.accessors
            .ok_or( Error::GLTFJsonError( "glTF does not contain any accessors!".to_owned() ) )?;

        let mut mesh_buffer:Vec<Rc<Self>> = Vec::new();
        for mesh in meshes.iter() {

            for primitive in mesh.primitives().iter() {

                let mut data_order = Vec::new();

                let mut collect_vertices = | data_type:MeshData | -> Result<Vec<Vec<f32>>, Error>
                {
                    let accessor = match data_type {
                        MeshData::Position(_) => &accessors[primitive.attributes().position()],
                        MeshData::Normal(_)   => &accessors[primitive.attributes().normal()],
                        MeshData::Tangent(_)  => {
                            let tangent_idx = primitive.attributes().tangent()
                                .expect( "no tangents?" );
                            &accessors[tangent_idx]
                        },
                        _ => {
                            let uv_idx = primitive.attributes().tex_coord(0)
                                .expect( "no uv?" );
                            &accessors[uv_idx]
                        },
                    };
                    let buffer_view_idx = accessor.buffer_view().ok_or(
                            Error::GLTFJsonError( "glTF accessor does not contain buffer view index!".to_owned() )
                    )?;
                    let buffer_view = &buffer_views[buffer_view_idx];
                    let bytes = {
                        let offset = buffer_view.byte_offset();
                        let len = buffer_view.byte_length();
                        &buffer_bytes[buffer_view.buffer()][ offset..( offset + len ) ]
                    };
                    let temp_buffer_size = data_type.component_count();
                    let mut vertex_buffer:Vec<Vec<f32>> = Vec::with_capacity(
                        bytes.len() / temp_buffer_size * size_of::<f32>() );
                    let mut temp_buffer:Vec<f32> = vec![0f32;temp_buffer_size];
                    let mut temp_buffer_idx = 0;
                    for float_bytes in bytes.chunks( size_of::<f32>() ) {
                        let float = f32::from_le_bytes(
                            float_bytes.try_into()
                                .map_err(
                                    |e|
                                        Error::GLTFJsonError( format!( "Incorrect slice length! {}", e ) )
                                )?
                        );
                        temp_buffer[temp_buffer_idx] = float;
                        temp_buffer_idx += 1;
                        if temp_buffer_idx >= temp_buffer_size {
                            vertex_buffer.push( temp_buffer.clone() );
                            temp_buffer_idx = 0;
                        }
                    }
                    match data_type {
                        MeshData::Position(_) => data_order.push( MeshData::Position( vertex_buffer.len() ) ),
                        MeshData::Normal(_)   => data_order.push( MeshData::Normal( vertex_buffer.len() ) ),
                        MeshData::Tangent(_)  => data_order.push( MeshData::Tangent( vertex_buffer.len() ) ),
                        MeshData::Color(_)    => data_order.push( MeshData::Color( vertex_buffer.len() ) ),
                        MeshData::UV(_)       => data_order.push( MeshData::UV( vertex_buffer.len() ) ),
                    }
                    Ok( vertex_buffer )
                };

                let collect_vertex_color = | data_order:&mut Vec<MeshData> | -> Result< Option< Vec<Vec<f32> > >, Error > {
                    let accessor = match primitive.attributes().color(0) {
                        Some(a) => &accessors[a],
                        None => return Ok( None ),
                    };
                    let buffer_view_idx = accessor.buffer_view().ok_or(
                        Error::GLTFJsonError( "glTF accessor does not contain buffer view index!".to_owned() )
                    )?;
                    let buffer_view = &buffer_views[buffer_view_idx];
                    let bytes = {
                        let offset = buffer_view.byte_offset();
                        let len = buffer_view.byte_length();
                        &buffer_bytes[buffer_view.buffer()][ offset..( offset + len ) ]
                    };
                    let temp_buffer_size = MeshData::Color(0).component_count();
                    let mut vertex_buffer:Vec<Vec<f32>> = Vec::with_capacity(
                        bytes.len() / temp_buffer_size * size_of::<f32>() );
                    let mut temp_buffer:Vec<f32> = vec![0f32;temp_buffer_size];
                    let mut temp_buffer_idx = 0;
                    for short_bytes in bytes.chunks( size_of::<u16>() ) {
                        let short = u16::from_le_bytes(
                            short_bytes.try_into()
                                .map_err(
                                    |e|
                                        Error::GLTFJsonError( format!( "Incorrect slice length! {}", e ) )
                                )?
                        );
                        temp_buffer[temp_buffer_idx] = ( short as f32 ) / u16::MAX as f32;
                        temp_buffer_idx += 1;
                        if temp_buffer_idx >= temp_buffer_size {
                            vertex_buffer.push( temp_buffer.clone() );
                            temp_buffer_idx = 0;
                        }
                    }
                    data_order.push( MeshData::Color( vertex_buffer.len() ) );
                    Ok( Some( vertex_buffer ) )
                };

                let positions = collect_vertices( MeshData::Position(0) )?;
                let normals   = collect_vertices( MeshData::Normal(0) )?;
                let tangents = match &primitive.attributes().tangent() {
                    Some(_) => Some(collect_vertices( MeshData::Tangent(0) )?),
                    None => None,
                };
                let uvs = match &primitive.attributes().tex_coord( 0 ) {
                    Some(_) => Some(collect_vertices( MeshData::UV(0) )?),
                    None => None,
                };
                let colors = collect_vertex_color( &mut data_order )?;

                let indices = {
                    let accessor = &accessors[
                        primitive.indices()
                            .ok_or(
                                Error::GLTFJsonError(
                                    "glTF primitive does not contain indices!".to_owned()
                                )
                        )? as usize];
                    let buffer_view = &buffer_views[
                        accessor.buffer_view()
                            .ok_or(
                                Error::GLTFJsonError(
                                    "glTF accessor does not contain buffer view index!".to_owned()
                                )
                            )?
                        as usize
                    ];
                    let bytes = {
                        let offset = buffer_view.byte_offset() as usize;
                        let len = buffer_view.byte_length() as usize;
                        &buffer_bytes[buffer_view.buffer() as usize][ offset..( offset + len ) ]
                    };

                    let mut indices_buffer:Vec<GLuint> = Vec::with_capacity( accessor.count() as usize );

                    for byte in bytes.chunks( 2 ) {
                        let index = u16::from_le_bytes(
                            byte.try_into()
                            .map_err(
                                |e|
                                    Error::GLTFJsonError( format!( "Incorrect slice length! {}", e ) )
                            )?
                        );
                        indices_buffer.push( index as GLuint );
                    }
                    indices_buffer
                };

                let vertices = {

                    let capacity = {
                        positions.len() + normals.len() +
                        match &tangents {
                            Some(t) => t.len(),
                            None => 0,
                        } +
                        match &uvs {
                            Some(uv) => uv.len(),
                            None => 0,
                        } +
                        match &colors {
                            Some(c) => c.len(),
                            None => 0,
                        }
                    };

                    let mut buffer:Vec<f32> = Vec::with_capacity( capacity );

                    buffer.extend( positions.into_iter().flatten().collect::<Vec<f32>>() );
                    buffer.extend( normals.into_iter().flatten().collect::<Vec<f32>>() );

                    match tangents {
                        Some(t) =>
                            buffer.extend( t.into_iter().flatten().collect::<Vec<f32>>() ),
                        None => {},
                    }

                    match uvs {
                        Some(u) =>
                            buffer.extend( u.into_iter().flatten().collect::<Vec<f32>>() ),
                        None => {},
                    }

                    match colors {
                        Some(c) => 
                            buffer.extend( c.into_iter().flatten().collect::<Vec<f32>>() ),
                        None => {},
                    }

                    buffer
                };

                let culling = {
                    match primitive.material() {
                        Some(m) => {
                            match &gltf.materials {
                                Some(materials) => {
                                    if materials[m].double_sided() {
                                        Culling::initialize_disabled()
                                    } else { Culling::initialize() }
                                },
                                None => Culling::initialize(),
                            }
                        },
                        None => Culling::initialize(),
                    }
                };

                mesh_buffer.push(
                    Self::from_raw(
                        vertices, indices,
                        culling,
                        data_order, false
                    )
                )
            }
        }
        return Ok( mesh_buffer );
    }

    pub fn from_raw(
        vertices:Vec<f32>, indices:Vec<GLuint>,
        culling:Culling, data_order:Vec<MeshData>,
        interleaved:bool
    ) -> Rc<Self> {
        
        let index_count = indices.len();
        let vertex_size = ( vertices.len() * size_of::<f32>() ) as GLsizeiptr;
        let index_size  = ( index_count    * size_of::<GLuint>() ) as GLsizeiptr;

        let mut vao = 0;
        let mut vbo = 0;
        let mut ebo = 0;
        unsafe {
            // vertex array object
            gl::GenVertexArrays( 1, &mut vao );
            gl::BindVertexArray( vao );

            // vertex buffer object
            gl::GenBuffers( 1, &mut vbo );
            gl::BindBuffer( gl::ARRAY_BUFFER, vbo );
            gl::BufferData(
                gl::ARRAY_BUFFER,
                vertex_size,
                vertices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            // element buffer object
            gl::GenBuffers( 1, &mut ebo );
            gl::BindBuffer( gl::ELEMENT_ARRAY_BUFFER, ebo );
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                index_size,
                indices.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW
            );

            if interleaved {

                let stride = {
                    let mut buffer = 0;
                    for data in data_order.iter() {
                        buffer += data.byte_size();
                    } buffer
                };

                let mut ptr_offset = 0;

                for (idx, data) in data_order.iter().enumerate() {
                    gl::EnableVertexAttribArray( idx as GLuint );
                    gl::VertexAttribPointer(
                        idx as GLuint, data.component_count() as GLint, gl::FLOAT,
                        gl::FALSE, stride as GLsizei,
                        ptr_offset as *const GLvoid
                    );
                    ptr_offset = data.byte_size();
                }

            } else {
                
                let mut ptr_offset = 0;

                for (idx, data) in data_order.iter().enumerate() {
                    gl::EnableVertexAttribArray( idx as GLuint );
                    gl::VertexAttribPointer(
                        idx as GLuint, data.component_count() as GLint, gl::FLOAT,
                        gl::FALSE, 0,
                        ptr_offset as *const GLvoid
                    );
                    ptr_offset += data.count() * data.byte_size();
                }

            }

        }

        Rc::new(
            Self {
                vertices, indices,
                culling, vao, vbo, ebo,
                index_count
            }
        )
    }

    pub fn bind_buffers( &self ) {
        unsafe {
            gl::BindVertexArray( self.vertex_array_object() );
            gl::BindBuffer( gl::ARRAY_BUFFER, self.vertex_buffer_object() );
            gl::BindBuffer( gl::ELEMENT_ARRAY_BUFFER, self.element_buffer_object() );
        }
    }

    pub fn draw( &self ) {
        unsafe {
            gl::DrawElements(
                gl::TRIANGLES,
                self.index_count as GLsizei,
                gl::UNSIGNED_INT,
                core::ptr::null_mut() as *const GLvoid
            );
        }
    }

    pub fn render(&self) {
        self.culling().update_gl();
        self.bind_buffers();
        self.draw();
    }

    pub fn vertices(&self) -> &[f32]    { &self.vertices }
    pub fn indices(&self)  -> &[GLuint] { &self.indices }

    pub fn culling(&self) -> &Culling { &self.culling }
    pub fn culling_mut(&mut self) -> &mut Culling { &mut self.culling }

    pub fn vertex_array_object(&self)   -> GLuint { self.vao }
    pub fn vertex_buffer_object(&self)  -> GLuint { self.vbo }
    pub fn element_buffer_object(&self) -> GLuint { self.ebo }

    pub fn vertex_count(&self)   -> usize { self.index_count }
    pub fn triangle_count(&self) -> usize { self.index_count / 3 }

    pub fn delete_meshes( meshes:Vec<Rc<Self>> ) {
        let mut vertex_arrays:Vec<GLuint> = Vec::with_capacity( meshes.len() );
        let mut buffers:Vec<GLuint>       = Vec::with_capacity( meshes.len() * 2 );
        for mesh in meshes.iter() {
            vertex_arrays.push( mesh.vao );
            buffers.push( mesh.vbo );
            buffers.push( mesh.ebo );
        }
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

        drop( meshes );
    }

}

pub enum MeshData {
    Position(usize),  // 3 f32
    Normal  (usize),  // 3 f32
    Tangent (usize),  // 4 f32
    Color   (usize),  // 4 f32
    UV      (usize),  // 2 f32
}

impl MeshData {
    pub fn component_count(&self) -> usize {
        match self {
            Self::Color(_)   |
            Self::Tangent(_) => 4,

            Self::UV     (_) => 2,

            _ => 3,
        }
    }

    pub fn byte_size(&self) -> usize {
        self.component_count() * size_of::<f32>()
    }

    pub fn count(&self) -> usize {
        match self {
            Self::Position(c) |
            Self::Normal(c)   |
            Self::Tangent(c)  |
            Self::Color(c)    |
            Self::UV(c)       => *c,
        }
    }

    pub fn msg(&self) -> &str {
        match self {
            Self::Position(_) => "Position",
            Self::Normal  (_) => "Normal",
            Self::Tangent (_) => "Tangent",
            Self::Color   (_) => "Color",
            Self::UV      (_) => "UV",
        }
    }
}

impl fmt::Display for MeshData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}

// impl Mesh {

//     pub fn new_with_culling( vertices: Vec<Vertex>, indeces: Vec<GLuint>, culling: Culling ) -> Self {
//         const VERTEX_SIZE:GLint = 32;
//         const U32_SIZE:GLint    =  4;
//         const NORMAL_PTR_OFFSET:GLint = 12;
//         const UV_PTR_OFFSET:GLint     = 24;

//         let index_count = indeces.len() as GLint;

//         let vertices_size = ( vertices.len() * VERTEX_SIZE as usize ) as GLsizeiptr;
//         let indeces_size  = ( index_count * U32_SIZE ) as GLsizei;

//         let mut vbo:GLuint = 0;
//         let mut vao:GLuint = 0;
//         let mut ebo:GLuint = 0;

//         unsafe {
//             // vertex array object
//             gl::GenVertexArrays(1, &mut vao);
//             gl::BindVertexArray(vao);
    
//             // vertex buffer object
//             gl::GenBuffers(1, &mut vbo);
//             gl::BindBuffer( gl::ARRAY_BUFFER, vbo );
//             gl::BufferData(
//                 gl::ARRAY_BUFFER,
//                 vertices_size,
//                 vertices.as_ptr() as *const GLvoid,
//                 gl::STATIC_DRAW
//             );
    
//             // element buffer object
//             gl::GenBuffers(1, &mut ebo);
//             gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
//             gl::BufferData(
//                 gl::ELEMENT_ARRAY_BUFFER,
//                 indeces_size as GLsizeiptr,
//                 indeces.as_ptr() as *const GLvoid,
//                 gl::STATIC_DRAW
//             );

//             // position
//             gl::EnableVertexAttribArray( 0 );
//             gl::VertexAttribPointer(
//                 0, 3, gl::FLOAT,
//                 gl::FALSE, VERTEX_SIZE,
//                 0 as *const GLvoid
//             );
//             // normals
//             gl::EnableVertexAttribArray( 1 );
//             gl::VertexAttribPointer(
//                 1, 3, gl::FLOAT,
//                 gl::FALSE, VERTEX_SIZE,
//                 NORMAL_PTR_OFFSET as *const GLvoid,
//             );
//             // uvs
//             gl::EnableVertexAttribArray( 2 );
//             gl::VertexAttribPointer(
//                 2, 2, gl::FLOAT,
//                 gl::FALSE, VERTEX_SIZE,
//                 UV_PTR_OFFSET as *const GLvoid,
//             );

//         }

//         Self {
//             vertices,
//             indeces,
//             culling,
//             vao, vbo, ebo,
//             index_count,
//         }
//     }

// }

// #[allow(dead_code)]
// pub fn generate_plane() -> Mesh {

//     let vertices:Vec<Vertex> = vec![
//         Vertex{ position:Vector3::new(-0.5, 0.5,0.0), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,1.0) },
//         Vertex{ position:Vector3::new(-0.5,-0.5,0.0), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,0.0) },
//         Vertex{ position:Vector3::new( 0.5, 0.5,0.0), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,1.0) },
//         Vertex{ position:Vector3::new( 0.5,-0.5,0.0), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,0.0) },
//     ];

//     let indeces:Vec<GLuint> = vec![
//         0, 1, 2,
//         2, 1, 3,
//     ];

//     Mesh::new_with_culling( vertices, indeces, Culling::initialize_disabled() )

// }

// #[allow(dead_code)]
// pub fn generate_cube() -> Mesh {
//     let vertices:Vec<Vertex> = vec![
//         // front
//         // top left
//         Vertex{ position:Vector3::new(-0.5, 0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,1.0)},
//         // top right
//         Vertex{ position:Vector3::new( 0.5, 0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,1.0)},
//         // bottom left
//         Vertex{ position:Vector3::new(-0.5,-0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(0.0,0.0)},
//         // bottom right
//         Vertex{ position:Vector3::new( 0.5,-0.5,0.5), normal:Vector3::new(0.0,0.0,1.0), uv:Vector2::new(1.0,0.0)},

//         // back
//         // top left
//         Vertex{ position:Vector3::new( 0.5, 0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(1.0,1.0)},
//         // top right
//         Vertex{ position:Vector3::new(-0.5, 0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(0.0,1.0)},
//         // bottom left
//         Vertex{ position:Vector3::new( 0.5,-0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(1.0,0.0)},
//         // bottom right
//         Vertex{ position:Vector3::new(-0.5,-0.5,-0.5), normal:Vector3::new(0.0,0.0,-1.0), uv:Vector2::new(0.0,0.0)},

//         // left
//         // top left
//         Vertex{ position:Vector3::new(-0.5, 0.5,-0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(0.0,1.0)},
//         // top right
//         Vertex{ position:Vector3::new(-0.5, 0.5, 0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(1.0,1.0)},
//         // bottom left
//         Vertex{ position:Vector3::new(-0.5,-0.5,-0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(0.0,0.0)},
//         // bottom right
//         Vertex{ position:Vector3::new(-0.5,-0.5, 0.5), normal:Vector3::new(-1.0,0.0,0.0), uv:Vector2::new(1.0,0.0)},

//         // right
//         // top left
//         Vertex{ position:Vector3::new(0.5,  0.5,  0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(1.0,1.0)},
//         // top right
//         Vertex{ position:Vector3::new(0.5,  0.5, -0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(0.0,1.0)},
//         // bottom left
//         Vertex{ position:Vector3::new(0.5, -0.5,  0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(1.0,0.0)},
//         // bottom right
//         Vertex{ position:Vector3::new(0.5, -0.5, -0.5), normal:Vector3::new(1.0,0.0,0.0), uv:Vector2::new(0.0,0.0)},

//         // top
//         Vertex{ position:Vector3::new( 0.5,  0.5,  0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(1.0,1.0)},
//         Vertex{ position:Vector3::new(-0.5,  0.5,  0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(0.0,1.0)},
//         Vertex{ position:Vector3::new( 0.5,  0.5, -0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(1.0,0.0)},
//         Vertex{ position:Vector3::new(-0.5,  0.5, -0.5), normal:Vector3::new(0.0,1.0,0.0), uv:Vector2::new(0.0,0.0)},

//         // bottom
//         Vertex{ position:Vector3::new(-0.5, -0.5,  0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(0.0,1.0)},
//         Vertex{ position:Vector3::new( 0.5, -0.5,  0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(1.0,1.0)},
//         Vertex{ position:Vector3::new(-0.5, -0.5, -0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(0.0,0.0)},
//         Vertex{ position:Vector3::new( 0.5, -0.5, -0.5), normal:Vector3::new(0.0,-1.0,0.0), uv:Vector2::new(1.0,0.0)},
//     ];

//     let indeces:Vec<u32> = vec![
//         0, 2, 1,
//         1, 2, 3,

//         4, 6, 5,
//         5, 6, 7,

//         8, 10,  9,
//         9, 10, 11,

//         12, 14, 13,
//         13, 14, 15,

//         16, 18, 17,
//         17, 18, 19,

//         20, 22, 21,
//         21, 22, 23,
//     ];

//     Mesh::new( vertices, indeces )
// }