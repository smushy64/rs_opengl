use gl::types::*;
use crate::{ Rc, debugging::Error };
use core::fmt;
use fmath::types::color::RGB;

#[derive(Debug)]
pub struct Texture {
    handle:GLuint,
    image:ImageGL,
    options:TextureOptions
}

impl Texture {

    pub fn empty() -> Rc<Self> {
        Rc::new( Self {
            handle: 0, image:ImageGL::empty(),
            options:TextureOptions::default()
        } )
    }

    pub fn new_color_texture( color:RGB ) -> Rc<Self> {
        let image = ImageGL {
            width: 1, height: 1,
            format: gl::RGB,
            data: Vec::from( color.as_array_rgb() )
        };
        let options = TextureOptions::default();
        Self::new( image, options )
    }

    pub fn new( image:ImageGL, options:TextureOptions ) -> Rc<Self> {
        let mut handle = 0;

        unsafe {

            gl::GenTextures( 1, &mut handle );
            gl::BindTexture( gl::TEXTURE_2D, handle );
                
            match options.border_color {
                Some(c) => {
                    let col = c.as_array_rgba_f32();
                    gl::TexParameterfv(
                        gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR,
                        col.as_ptr()
                    );
                },
                None => {},
            }

            gl::TexParameteri(
                gl::TEXTURE_2D, gl::TEXTURE_WRAP_S,
                options.wrapping_x.as_glint()
            );
            gl::TexParameteri(
                gl::TEXTURE_2D, gl::TEXTURE_WRAP_T,
                options.wrapping_y.as_glint()
            );

            gl::TexParameteri(
                gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,
                options.min_filtering.as_glint()
            );
            gl::TexParameteri(
                gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER,
                options.mag_filtering.as_glint()
            );

            gl::TexImage2D(
                gl::TEXTURE_2D, 0, image.format as GLint,
                image.width, image.height,
                0, image.format, gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const GLvoid
            );

            gl::GenerateMipmap( gl::TEXTURE_2D );

        }

        Rc::new( Texture { handle, image, options } )
    }

    pub fn handle( &self )        -> &GLuint           { &self.handle                }
    pub fn width( &self )         -> &GLint            { &self.image.width           }
    pub fn height( &self )        -> &GLint            { &self.image.height          }
    pub fn image_data( &self )    -> &Vec<u8>          { &self.image.data            }
    pub fn wrapping_x( &self )    -> &TextureWrapping  { &self.options.wrapping_x    }
    pub fn wrapping_y( &self )    -> &TextureWrapping  { &self.options.wrapping_y    }
    pub fn min_filtering( &self ) -> &MipmapFiltering  { &self.options.min_filtering }
    pub fn mag_filtering( &self ) -> &TextureFiltering { &self.options.mag_filtering }

    pub fn use_texture( &self, sampler:&Sampler, uniform_handle:GLint ) {
        unsafe {
            gl::ActiveTexture( sampler.handle() );
            gl::BindTexture( gl::TEXTURE_2D, *self.handle() );
            gl::Uniform1i( uniform_handle, *sampler.id() );
        }
    }

}

impl fmt::Display for Texture {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "Texture {} | width: {} height: {} size (bytes): {}",
            self.handle(), self.width(), self.height(), self.image_data().len()
        )
    }
}

pub unsafe fn delete_textures( textures: Vec<Rc<Texture>> ) {

    let mut handles:Vec<GLuint> = Vec::with_capacity( textures.len() );
    for texture in textures.iter() {
        let ref_count = Rc::strong_count( texture );
        if ref_count != 1 {
            // make sure each texture being deleted is not in use
            panic!( "Attempted to delete a texture that is still in use! Reference Count: {}", ref_count );
        }
        handles.push( *texture.handle() );
    }
    drop( textures );
    gl::DeleteTextures( handles.len() as GLsizei, handles.as_ptr() );
}

#[derive(Clone, Debug)]
pub struct Sampler { id:GLint }

impl Sampler {

    pub fn new( id:GLint ) -> Self { Self{ id } }
    pub fn empty() -> Self { Self{ id:0 } }

    pub fn handle( &self ) -> GLenum {
        gl::TEXTURE0 + (self.id as GLuint)
    }

    pub fn id( &self ) -> &GLint { &self.id }

}

impl fmt::Display for Sampler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "TEXTURE {}", self.id() )
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextureOptions {
    wrapping_x:TextureWrapping, wrapping_y:TextureWrapping,
    min_filtering:MipmapFiltering, mag_filtering:TextureFiltering,
    border_color:Option<RGB>
}

impl TextureOptions {
    pub fn default() -> Self {
        Self {
            wrapping_x: TextureWrapping::Repeat, wrapping_y: TextureWrapping::Repeat,
            min_filtering: MipmapFiltering::LinearLinear, mag_filtering: TextureFiltering::Linear,
            border_color:None
        }
    }

    pub fn set_border_color( &mut self, c:RGB ) {
        self.border_color = Some(c);
    }

    pub fn set_wrapping( &mut self, wrapping:TextureWrapping ) {
        self.wrapping_x = wrapping;
        self.wrapping_y = wrapping;
    }

    pub fn set_wrapping_x( &mut self, wrapping:TextureWrapping ) {
        self.wrapping_x = wrapping;
    }

    pub fn set_wrapping_y( &mut self, wrapping:TextureWrapping ) {
        self.wrapping_y = wrapping;
    }

    pub fn set_min_filtering( &mut self, filtering:MipmapFiltering ) {
        self.min_filtering = filtering;
    }

    pub fn set_mag_filtering( &mut self, filtering:TextureFiltering ) {
        self.mag_filtering = filtering;
    }

}

#[derive(Clone, Copy, Debug)]
pub enum TextureWrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder,
}

impl TextureWrapping {
    pub fn as_glint( &self ) -> GLint {
        match self {
            Self::Repeat           => gl::REPEAT          as GLint,
            Self::MirroredRepeat   => gl::MIRRORED_REPEAT as GLint,
            Self::ClampToEdge      => gl::CLAMP_TO_EDGE   as GLint,
            Self::ClampToBorder => gl::CLAMP_TO_BORDER as GLint,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TextureFiltering {
    Nearest,
    Linear,
}

impl TextureFiltering {
    pub fn as_glint(&self) -> GLint {
        match self {
            Self::Nearest => gl::NEAREST as GLint,
            Self::Linear  => gl::LINEAR  as GLint,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MipmapFiltering {
    NearestNearest,
    LinearNearest,
    NearestLinear,
    LinearLinear,
}

impl MipmapFiltering {
    pub fn as_glint(&self) -> GLint {
        match self {
            Self::NearestNearest => gl::NEAREST_MIPMAP_NEAREST as GLint,
            Self::LinearNearest  => gl::LINEAR_MIPMAP_NEAREST  as GLint,
            Self::NearestLinear  => gl::NEAREST_MIPMAP_LINEAR  as GLint,
            Self::LinearLinear   => gl::LINEAR_MIPMAP_LINEAR   as GLint,
        }
    }
}

#[derive(Debug)]
pub struct ImageGL {
    pub width:GLint, pub height:GLint,
    pub format:GLenum,
    pub data:Vec<u8>
}

impl ImageGL {

    pub fn empty() -> Self {
        Self {
            width:  0, height: 0,
            format: 0, data: Vec::new()
        }
    }

    pub fn from_dynamic_image( dynamic_image: image::DynamicImage ) -> Result<Self, Error> {

        let dynamic = dynamic_image.flipv();

        let ( format, data) = match dynamic.color() {
            image::ColorType::Rgb8  => ( gl::RGB,  dynamic.to_rgb8().as_raw().clone()  ),
            image::ColorType::Rgba8 => ( gl::RGBA, dynamic.to_rgba8().as_raw().clone() ),
            _ => return Err( Error::TextureUnsupportedColorFormat ),
        };

        Ok( 
            Self {
                width: dynamic.width() as GLint,
                height: dynamic.height() as GLint,
                format, data
            }
        )

    }

}
