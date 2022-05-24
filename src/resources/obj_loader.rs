use fmath::types::*;
use crate::graphics::{
    Mesh, Vertex
};
use super::Error;

const COMMENT:char = '#';

const POSITION:&str = "v";
const UV:&str       = "vt";
const NORMAL:&str   = "vn";
const INDEX:&str    = "f";

pub struct MeshOBJ {
    pub positions: Vec<Vector3>,
    pub normals:   Vec<Vector3>,
    pub uvs:       Vec<Vector2>,
    pub colors:    Vec<Vector3>,
    pub faces:     Vec<(u32, u32, u32)>,
}

impl MeshOBJ {
    pub fn new_empty() -> Self {
        Self {
            positions: Vec::new(),
            normals:   Vec::new(),
            uvs:       Vec::new(),
            colors:    Vec::new(),
            faces:     Vec::new(),
        }
    }
}

impl core::fmt::Display for MeshOBJ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let msg_gen_v3 = |vec:&Vec<Vector3>, kind:&str| -> String {
            let mut msg = String::new();
            for p in vec.iter() {
                msg.push_str( &format!("{} {}\n", kind, p ) );
            }
            msg
        };

        let msg_gen_v2 = |vec:&Vec<Vector2>| -> String {
            let mut msg = String::new();
            for p in vec.iter() {
                msg.push_str( &format!("vt {}\n", p ) );
            }
            msg
        };

        let msg_gen_i = |vec:&Vec<(u32, u32, u32)>| -> String {
            let mut msg = String::from("f ");
            let mut counter = 0;
            for p in vec.iter() {
                if counter >= 3 {
                    counter = 0;
                    msg.push_str( "\nf " );
                }
                msg.push_str( &format!("{}/{}/{} ", p.0, p.1, p.2, ) );
                counter += 1;
            }
            msg
        };

        let pos = msg_gen_v3( &self.positions, "v" );
        let uvs = msg_gen_v2( &self.uvs );
        let norm = msg_gen_v3( &self.normals, "vn" );
        let col = if self.colors.is_empty() { format!("") }
            else { msg_gen_v3( &self.colors, "c" ) };
        let i = msg_gen_i( &self.faces );
        
        write!( f, "{}{}{}{}{}", pos, uvs, norm, col, i )
    }
}

pub fn parse_obj( raw:String ) -> Result<Vec<MeshOBJ>, Error> {

    let parser =
    | lines:Vec<&str>, index_offset:( u32, u32, u32 ) | -> Result<( MeshOBJ, ( u32, u32, u32 ) ), Error> {
        let mut mesh = MeshOBJ::new_empty();
        let mut next_index_offset = ( 0u32, 0u32, 0u32 );
        // parse data out of text
        // iterate through each line
        for line in lines.iter() {
            // skip empty lines
            if line.is_empty() { continue; }
            // skip comment lines
            if line.contains( COMMENT ) { continue; }
    
            // get symbols
            let symbols:Vec<&str> = line.split_whitespace().collect();
    
            let parse_floats = | symbols:Vec<&str> | -> Result<Vec<f32>, Error> {
                let mut result = Vec::new();
                for symbol in symbols.iter().skip(1) {
    
                    let data = match symbol.parse::<f32>() {
                        Ok(f) => f,
                        Err(e) => return Err(
                            Error::OBJParse( format!("Parse .obj Error: {}", e) )
                        ),
                    };
    
                    result.push( data );
                }
                Ok( result )
            };
    
            // only parse if first symbol is recognized
            match symbols[0] {
                POSITION => {
                    let result = parse_floats( symbols )?;
                    match result.len() {
                        3 => { // position
                            mesh.positions.push( Vector3::new( result[0], result[1], result[2] ) );
                        },
                        6 => { // position + color
                            mesh.positions.push( Vector3::new( result[0], result[1], result[2] ) );
                            mesh.colors.push(    Vector3::new( result[3], result[4], result[5] ) );
                        }
                        _ => { // error
                            return Err(
                                Error::OBJParse(
                                    format!( "Parse .obj Error: Positions/Vertex Colors are not properly formatted!" )
                                )
                            );
                        }
                    }
                    
                }
                UV       => {
                    let result = parse_floats( symbols )?;
                    match result.len() {
                        2 => {
                            mesh.uvs.push( Vector2::new( result[0], result[1] ) );
                        }
                        _ => {
                            return Err(
                                Error::OBJParse(
                                    format!( "Parse .obj Error: UVs are not properly formatted!" )
                                )
                            );
                        }
                    }
                }
                NORMAL   => {
                    let result = parse_floats( symbols )?;
                    match result.len() {
                        3 => {
                            mesh.normals.push( Vector3::new( result[0], result[1], result[2] ) );
                        }
                        _ => {
                            return Err(
                                Error::OBJParse(
                                    format!( "Parse .obj Error: Normals are not properly formatted!" )
                                )
                            );
                        }
                    }
                }
                INDEX    => {
                    for symbol in symbols.iter().skip(1) {
                        let mut result:Vec<u32> = Vec::new();
                        let sub_symbols:Vec<&str> = symbol.split('/').collect();
    
                        for sub_symbol in sub_symbols {
                            let data = match sub_symbol.parse::<u32>() {
                                Ok(u) => u,
                                Err(e) => return Err(
                                    Error::OBJParse(
                                        format!( "Parse .obj Error: {}", e )
                                    )
                                ),
                            };
                            result.push( data );
                        }
    
                        match result.len() {
                            3 => {
                                if result[0] > next_index_offset.0 { next_index_offset.0 = result[0] }
                                if result[1] > next_index_offset.1 { next_index_offset.1 = result[1] }
                                if result[2] > next_index_offset.2 { next_index_offset.2 = result[2] }
                                mesh.faces.push(
                                    (
                                        result[0] - index_offset.0,
                                        result[1] - index_offset.1,
                                        result[2] - index_offset.2
                                    )
                                );
                            },
                            _ => {
                                return Err(
                                    Error::OBJParse(
                                        format!( "Parse .obj Error: Face Indeces are not properly formatted!" )
                                    )
                                )
                            }
                        }
    
                    }
                }
                _ => { continue; }
            };
    
        }
        Ok(( mesh, next_index_offset ))
    };

    let mut object_buffer:Vec<MeshOBJ> = Vec::new();
    // face indeces keep going up, they don't reset to 1 when in a new object -_-
    // offset is used to correct indeces when reading MeshOBJ
    let mut last_index_offset = ( 0, 0, 0 );

    let objects_raw:Vec<&str> = raw.split( "o " ).collect();
    // skip first, just header comments
    for object_raw in objects_raw.iter().skip(1) {
        let lines:Vec<&str> = object_raw.split( '\n' ).collect();
        let ( obj, next_index_offset ) = parser( lines, last_index_offset )?;
        last_index_offset = next_index_offset;
        object_buffer.push( obj );
    }

    Ok( object_buffer )
    
}

pub fn object_to_glmesh( obj:MeshOBJ ) -> Mesh {

    let mut vertices:     Vec<Vertex> = Vec::with_capacity( obj.faces.len() );
    let mut mesh_indeces: Vec<u32>    = Vec::with_capacity( obj.faces.len() );

    // ( position, uv, normal )
    // -1 because obj indeces start at 1 not 0!
    for (idx, index) in obj.faces.iter().enumerate() {

        let pos_i    = (index.0 - 1) as usize;
        let uv_i     = (index.1 - 1) as usize;
        let normal_i = (index.2 - 1) as usize;

        let vertex = Vertex {
            position: obj.positions[pos_i].clone(),
            normal:   obj.normals[normal_i].clone(),
            uv:       obj.uvs[uv_i].clone(),
        };

        vertices.push( vertex );
        mesh_indeces.push( idx as u32 );

    }

    Mesh::new( vertices, mesh_indeces )
}

pub fn objects_to_glmeshes( objects:Vec<MeshOBJ> ) -> Vec<Mesh> {
    let mut mesh_buffer:Vec<Mesh> = Vec::with_capacity( objects.len() );
    for object in objects {
        mesh_buffer.push( object_to_glmesh( object ) );
    }
    mesh_buffer
}
