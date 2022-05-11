use crate::shader::{ShaderProgram, Shader};

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
    let shader_source = parse_shader_program( load_string_path(path)? )?;

    ShaderProgram::from_shaders(&[ shader_source.compiled_vertex()?, shader_source.compiled_fragment()? ])
        .map_err( |e| Error::ShaderError(format!("{:?}", e)) )
}

fn parse_shader_program( shader_program:String ) -> Result<ParsedShaderSource, Error> {

    // PPD - Pre-processor directive
    const PPD_VERTEX:&str   = "#vertex";
    const PPD_FRAGMENT:&str = "#fragment";
    const PPD_COMMENT_SINGLE:&str = "//";

    // split into lines
    let parts:Vec<&str> = shader_program.split('\n').collect();

    let mut vert = String::new();
    let mut frag = String::new();

    let mut shader_kind = ShaderParserKind::None;

    for part in parts.iter() {

        if part.contains(PPD_VERTEX) {
            shader_kind = ShaderParserKind::Vertex;
            continue;
        } else if part.contains(PPD_FRAGMENT) {
            shader_kind = ShaderParserKind::Fragment;
            continue;
        } else if part.contains(PPD_COMMENT_SINGLE) {
            continue;
        }

        match shader_kind {
            ShaderParserKind::Vertex => {
                vert.push_str(part);
                vert.push('\n');
            },
            ShaderParserKind::Fragment => {
                frag.push_str(part);
                frag.push('\n');
            },
            ShaderParserKind::None => continue,
        }

    }

    if vert.is_empty() || frag.is_empty() {
        return Err(
            Error::ShaderError(format!("Error: Shader is not formatted properly!"))
        );
    }

    Ok(
        ParsedShaderSource {
            vertex: CString::new( vert.as_str() )
                .map_err(|_| Error::CStringContainsNull(format!("Error: File contains null character!")))?,
            fragment: CString::new( frag.as_str() )
                .map_err(|_| Error::CStringContainsNull(format!("Error: File contains null character!")))?,
        }
    )

}

struct ParsedShaderSource {
    vertex:CString,
    fragment:CString,
}

impl ParsedShaderSource {
    pub fn get_vertex_source(&self) -> &CString {
        &self.vertex
    }

    pub fn get_fragment_source(&self) -> &CString {
        &self.fragment
    }

    pub fn compiled_vertex(&self) -> Result<Shader, Error> {
        Shader::vert_from_source(self.get_vertex_source())
            .map_err( |e| Error::ShaderError(format!( "{:?}", e )) )
    }

    pub fn compiled_fragment(&self) -> Result<Shader, Error> {
        Shader::frag_from_source(self.get_fragment_source())
            .map_err( |e| Error::ShaderError(format!( "{:?}", e )) )
    }
}

enum ShaderParserKind {
    Vertex,
    Fragment,
    None,
}