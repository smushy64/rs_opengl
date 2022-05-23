use fmath::types::*;
use crate::geometry::*;
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

pub fn parse_obj( raw:String ) -> Result<MeshOBJ, Error> {

    let lines:Vec<&str> = raw.split( '\n' ).collect();

    let mut positions: Vec<Vector3>         = Vec::new();
    let mut colors:    Vec<Vector3>         = Vec::new();
    let mut normals:   Vec<Vector3>         = Vec::new();
    let mut uvs:       Vec<Vector2>         = Vec::new();
    let mut faces:     Vec<(u32, u32, u32)> = Vec::new();

    // parse data out of text
    // iterate through each line
    for line in lines.iter() {
        // skip empty lines
        if line.is_empty() { continue; }
        // skip comment lines
        if line.contains( COMMENT ) { continue; }

        // get symbols
        let symbols:Vec<&str> = line.split_whitespace().collect();

        let parse_vectors = | symbols:Vec<&str> | -> Result<Vec<f32>, Error> {
            let mut result = Vec::new();
            for symbol in symbols.iter().skip(1) {

                let data = match symbol.parse::<f32>() {
                    Ok(f) => f,
                    Err(e) => return Err(
                        Error::OBJParseError( format!("f32 parse error: {}", e) )
                    ),
                };

                result.push( data );
            }
            Ok( result )
        };

        // only parse if first symbol is recognized
        match symbols[0] {
            POSITION => {
                let result = parse_vectors( symbols )?;
                match result.len() {
                    3 => { // position
                        positions.push( Vector3::new( result[0], result[1], result[2] ) );
                    },
                    6 => { // position + color
                        positions.push( Vector3::new( result[0], result[1], result[2] ) );
                        colors.push(    Vector3::new( result[3], result[4], result[5] ) );
                    }
                    _ => { // error
                        return Err( Error::OBJParseError(format!( "Unable to parse .obj!" )) );
                    }
                }
                
            }
            UV       => {
                let result = parse_vectors( symbols )?;
                match result.len() {
                    2 => {
                        uvs.push( Vector2::new( result[0], result[1] ) );
                    }
                    _ => { return Err( Error::OBJParseError(format!( "Unable to parse .obj!" )) ); }
                }
            }
            NORMAL   => {
                let result = parse_vectors( symbols )?;
                match result.len() {
                    3 => {
                        normals.push( Vector3::new( result[0], result[1], result[2] ) );
                    }
                    _ => { return Err( Error::OBJParseError(format!( "Unable to parse .obj!" )) ); }
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
                                Error::OBJParseError(format!( "u32 parse error: {}", e ))
                            ),
                        };
                        result.push( data );
                    }

                    match result.len() {
                        3 => {
                            faces.push( ( result[0], result[1], result[2] ) );
                        },
                        _ => { return Err( Error::OBJParseError( format!( "Unable to parse .obj!" ) ) ) }
                    }

                }
            }
            _ => { continue; }
        };

    }

    Ok( 
        MeshOBJ {

            positions,
            normals,
            uvs,
            colors,
            faces

        }
    )

    
}

pub fn meshobj_to_meshgl( obj:MeshOBJ ) -> Mesh {

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
