#[allow(unused_imports)]
use std::{
    env, fs,
    path::{ PathBuf, Path },
    ffi::{ CString, CStr }
};

mod image_loader;
pub use image_loader::DynamicImage;

mod obj_loader;
pub use obj_loader::{ parse_obj, object_to_glmesh, objects_to_glmeshes };

mod shader_loader;
pub use shader_loader::{
    load_shader_program,
    load_shader_program_path
};

use crate::{ graphics::{ *, texture::{ TextureOptions, ImageGL } }, Rc };

static mut RESOURCES_PATH:String = String::new();
pub fn get_resources_path() -> PathBuf {
    unsafe { PathBuf::from( RESOURCES_PATH.clone() ) }
}

pub fn load_meshes( local_path:&str ) -> Result< Vec<Mesh>, Error > {

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
                    let objects = parse_obj( raw )?;
                    Ok( objects_to_glmeshes( objects ) )
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

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::ReadFile            (s) => s,
            Error::UTF8Conversion      (s) => s,
            Error::CStringContainsNull (s) => s,
            Error::LoadImage           (s) => s,
            Error::ShaderParse    (s) => s,
            Error::NoFileType          (s) => s,
            Error::UnrecognizedFileExt(s) => s,
            Error::OBJParse       (s) => s,
        };
        write!( f, "{}", msg )
    }
}

// recognized file extensions
const OBJ_EXT:&str = "obj";