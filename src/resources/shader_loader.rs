use crate::shaders::{ShaderProgram, Shader};

use super::{
    resource_path_from_local_path,
    load_string_path,
    CString, PathBuf, Error
};

pub fn load_shader_program( local_path:&str ) -> Result<ShaderProgram, Error> {
    let mut path = resource_path_from_local_path(local_path);
    path.set_extension("shader");
    load_shader_program_path(path)
}

pub fn load_shader_program_path( path:PathBuf ) -> Result<ShaderProgram, Error> {
    let cstring = parse_shader_program( load_string_path(path)? )?;

    ShaderProgram::from_shaders(&[
        Shader::vert_from_source(&cstring[0])
            .map_err(|e| Error::ShaderError(format!( "{:?}", e )))?,
        Shader::frag_from_source(&cstring[1])
            .map_err(|e| Error::ShaderError(format!( "{:?}", e )))?,
    ]).map_err( |e| Error::ShaderError(format!("{:?}", e)) )
}

fn parse_shader_program( shader_program:String ) -> Result<[CString; 2], Error> {

    const SPLIT_PATTERN:&str = "#fragment";

    let parts:Vec<&str> = shader_program.split( SPLIT_PATTERN ).collect();

    if parts.len() != 2 {
        return Err( Error::ShaderError( format!("Error: Shader is not properly formatted!") ) );
    }

    let vert = CString::new( parts[0] )
        .map_err(|_| Error::CStringContainsNull(format!("Error: File contains null character!")))?;
    let frag = CString::new( parts[1] )
        .map_err(|_| Error::CStringContainsNull(format!("Error: File contains null character!")))?;

    Ok( [ vert, frag ] )

}