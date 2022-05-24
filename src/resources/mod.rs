extern crate wavefront_obj;

#[allow(unused_imports)]
use std::{
    env, fs,
    path::{ PathBuf, Path },
    ffi::{ CString, CStr }
};
use crate::{
    graphics::{
        *, texture::{ TextureOptions, ImageGL },
        shader::shader_parser,
    }, Rc
};

mod image_loader;
pub use image_loader::DynamicImage;

static mut RESOURCES_PATH:String = String::new();
pub fn get_resources_path() -> PathBuf {
    unsafe { PathBuf::from( &RESOURCES_PATH ) }
}

pub fn load_program_info() -> Result<super::ProgramInfo, Error> {
    let path = resource_path_from_local_path( "program/program_info.txt" );
    let settings_src = load_string_path( &path )?;
    let lines:Vec<&str> = settings_src.split('\n').collect();
    let mut title = format!("OpenGL | ");
    let mut dimensions = fmath::types::Vector2::new_zero();
    for line in lines.iter() {
        if line.contains( "[title] " ) {
            let symbols:Vec<&str> = line.split("[title] ").collect();
            if symbols.len() < 2 {
                return Err(
                    Error::ReadFile( format!("Load Settings Error: Formatted incorrectly!") )
                )
            }
            title.push_str( symbols[1] );
        }
        if line.contains( "dimensions" ) {
            let symbols:Vec<&str> = line.split_whitespace().collect();
            if symbols.len() < 2 {
                return Err(
                    Error::ReadFile( format!("Load Settings Error: Formatted incorrectly!") )
                )
            }
            let v = match symbols[1].parse::<f32>() {
                Ok(res) => res,
                Err(e) => return Err(
                    Error::ReadFile( format!("Load Settings Error: {}", e) )
                ),
            };
            if line.contains(".x") {
                dimensions[0] = v;
            } else if line.contains(".y") {
                dimensions[1] = v;
            }
        }
    }
    return Ok( super::ProgramInfo{ title, dimensions } )
}

pub fn load_meshes( local_path:&str ) -> Result< Rc<Vec<Mesh>>, Error > {

    let path = resource_path_from_local_path( &format!( "models/{}", local_path ) );
    
    // determine file type
    match path.extension() {
        Some(ext) => {
            let ext_str = match ext.to_str() {
                Some(res) => res,
                None => return Err(
                    Error::UTF8Conversion(
                        format!( "Load Meshes Error: Failed to convert &OsStr to &str!" )
                    )
                ),
            };
            match ext_str {
                OBJ_EXT => {
                    let raw = load_string_path( &path )?;
                    let objects = wavefront_obj::parse_obj( raw )
                        .map_err( |e| Error::OBJParse( e.msg() ) )?;
                    let result = {
                        let mut mesh_buffer:Vec<Mesh> = Vec::with_capacity( objects.len() );
                        for object in objects {
                            let ( vertices_raw, indeces ) = object.as_opengl_format();
                            let vertices_formatted = unsafe {
                                Vertex::from_vec_unchecked( vertices_raw )
                            };
                            mesh_buffer.push( Mesh::new( vertices_formatted, indeces ) )
                        }
                        mesh_buffer
                    };
                    Ok( Rc::new( result ) )
                }
                _ => return Err(
                    Error::UnrecognizedFileExt(
                        format!("Load Meshes Error: \"{}\" is an unrecognized file extension!", ext_str)
                    )
                ),
            }
        },
        None => return Err(
            Error::NoFileType(
                format!("Load Meshes Error: No file type specified!")
            )
        ),
    }

}

pub fn load_texture( local_path:&str, options:TextureOptions ) -> Result<Rc<Texture>, Error> {
    load_texture_path(
        &resource_path_from_local_path( &format!( "textures/{}", local_path ) ),
        options
    )
}

pub fn load_texture_path( path:&PathBuf, options:TextureOptions ) -> Result<Rc<Texture>, Error> {
    let dynamic_image = load_image_path( path )?;
    let gl_image = ImageGL::from_dynamic_image( dynamic_image )
        .map_err( |e| Error::LoadImage( format!("{}", e) ) )?;
    Ok( Texture::new( gl_image, options ) )
}

pub fn load_image( local_path:&str ) -> Result<DynamicImage, Error> {
    load_image_path( &resource_path_from_local_path(local_path) )
}

pub fn load_image_path( path:&PathBuf ) -> Result<DynamicImage, Error> {
    image_loader::load_image(path)
        .map_err( |e| Error::LoadImage(e) )
}

pub fn load_shader_program( local_path:&str ) -> Result<Rc<ShaderProgram>, Error> {
    let mut path = resource_path_from_local_path( &format!( "shaders/{}", local_path ) );
    path.set_extension("shader");
    load_shader_program_path(&path)
}

pub fn load_shader_program_path( path:&PathBuf ) -> Result<Rc<ShaderProgram>, Error> {
    let shader_source = shader_parser( load_string_path(path)? )
        .map_err( |e| Error::ShaderParse(e.msg()) )?;
    ShaderProgram::from_shaders( &shader_source )
        .map_err( |e| Error::ShaderParse( e.msg() ) )
}

pub fn load_cstring( local_path:&str ) -> Result<CString, Error> {
    load_cstring_path( &resource_path_from_local_path(local_path) )
}

pub fn load_cstring_path( path:&PathBuf ) -> Result<CString, Error> {
    let bytes = load_bytes_path(path)?;
    for byte in bytes.iter() {
        if *byte == 0 {
            return Err(
                Error::CStringContainsNull(
                    format!("Load CString Error: File contains null character!")
                )
            )
        }
    }
    Ok( unsafe { CString::from_vec_unchecked( bytes ) } )
}

pub fn load_string( local_path:&str ) -> Result<String, Error> {
    load_string_path( &resource_path_from_local_path(local_path) )
}

pub fn load_string_path( path:&PathBuf ) -> Result<String, Error> {
    let bytes = load_bytes_path( path )?;
    Ok( 
        String::from( 
            core::str::from_utf8(&bytes)
                .map_err(
                    |e|
                    Error::UTF8Conversion(
                        format!("Load String Error: {}", e)
                    )
                )?
        )
    )
}

pub fn load_bytes( local_path:&str ) -> Result<Vec<u8>, Error> {
    load_bytes_path( &resource_path_from_local_path(local_path) )
}

pub fn load_bytes_path( path:&PathBuf ) -> Result<Vec<u8>, Error> {
    match fs::read( path ) {
        Ok(result) => Ok( result ),
        Err(error) => Err(
            Error::ReadFile(
                format!("Load Bytes Error, at path: {}\n{}", path.to_string_lossy().into_owned(), error)
            )
        ),
    }
}

const LOCAL_SEPARATOR:char = '/';
pub fn resource_path_from_local_path( local_path:&str ) -> PathBuf {
    let mut full_path = get_resources_path();

    if local_path.contains(LOCAL_SEPARATOR) {

        let parts:Vec<&str> = local_path.split(LOCAL_SEPARATOR).collect();

        for part in parts {
            full_path.push( part );
        }

    } else {
        full_path.push( local_path );
    }

    full_path
}

pub fn initialize() {
    unsafe {
        let exe_path = env::current_exe().unwrap();
        let mut resources = PathBuf::from(exe_path.parent().unwrap());
        resources.push("resources");
        RESOURCES_PATH = String::from(resources.to_str().unwrap());
    }
}

#[derive(Debug)]
pub enum Error {

    ReadFile(String),

    UTF8Conversion(String),

    CStringContainsNull(String),

    LoadImage(String),

    ShaderParse(String),

    NoFileType(String),
    UnrecognizedFileExt(String),

    OBJParse(String),

}

impl Error {
    pub fn msg( &self ) -> String {
        match self {
            Error::ReadFile            (s) => s.clone(),
            Error::UTF8Conversion      (s) => s.clone(),
            Error::CStringContainsNull (s) => s.clone(),
            Error::LoadImage           (s) => s.clone(),
            Error::ShaderParse         (s) => s.clone(),
            Error::NoFileType          (s) => s.clone(),
            Error::UnrecognizedFileExt (s) => s.clone(),
            Error::OBJParse            (s) => s.clone(),
        }
    }
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "{}", self.msg() )
    }
}

// recognized file extensions
const OBJ_EXT:&str = "obj";