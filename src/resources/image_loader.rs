extern crate image;

pub use image::DynamicImage;

#[allow(unused_imports)]
use std::{
    path::{ Path, PathBuf },
};

pub fn load_image( path:PathBuf ) -> Result<image::DynamicImage, String> {
    image::open( path )
        .map_err( |e| format!("{}", e) )
}