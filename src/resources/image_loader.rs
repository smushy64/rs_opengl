extern crate image;
pub use image::DynamicImage;
use std::path::PathBuf;
use crate::debugging::Error;

pub fn load_image( path:&PathBuf ) -> Result<image::DynamicImage, Error> {
    image::open( path )
        .map_err( |e| Error::ImageCrateLoad( format!("{}", e) ) )
}