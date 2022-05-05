#[allow(unused_imports)]
use std::{
    env, fs,
    path::{ PathBuf, Path },
    ffi::{ CString, CStr }
};

mod image_loader;
pub use image_loader::DynamicImage;

static mut RESOURCES_PATH:String = String::new();
pub fn get_resources_path() -> PathBuf {
    unsafe { PathBuf::from( RESOURCES_PATH.clone() ) }
}

pub fn load_image( local_path:&str ) -> Result<DynamicImage, Error> {
    load_image_path( resource_path_from_local_path(local_path) )
}

pub fn load_image_path( path:PathBuf ) -> Result<DynamicImage, Error> {
    image_loader::load_image(path)
        .map_err( |e| Error::LoadImage(e) )
}

pub fn load_cstring( local_path:&str ) -> Result<CString, Error> {
    load_cstring_path( resource_path_from_local_path(local_path) )
}

pub fn load_cstring_path( path:PathBuf ) -> Result<CString, Error> {
    let bytes = load_bytes_path(path)?;
    for byte in bytes.iter() {
        if *byte == 0 {
            return Err( Error::CStringContainsNull( format!("Error: File contains null character!") ) )
        }
    }
    Ok( unsafe { CString::from_vec_unchecked( bytes ) } )
}

pub fn load_string( local_path:&str ) -> Result<String, Error> {
    load_string_path( resource_path_from_local_path(local_path) )
}

pub fn load_string_path( path:PathBuf ) -> Result<String, Error> {
    let bytes = load_bytes_path(path)?;
    Ok( 
        String::from( 
            core::str::from_utf8(&bytes)
                .map_err( |e| Error::UTF8Conversion(format!("Error: {}", e)) )?
        )
    )
}

pub fn load_bytes( local_path:&str ) -> Result<Vec<u8>, Error> {
    load_bytes_path( resource_path_from_local_path(local_path) )
}

pub fn load_bytes_path( path:PathBuf ) -> Result<Vec<u8>, Error> {
    match fs::read( &path ) {
        Ok(result) => Ok( result ),
        Err(error) => Err(
            Error::ReadFile(format!("path: {} | Error: {}", path.to_string_lossy().into_owned(), error))
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
}