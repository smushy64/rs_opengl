use gl::types::*;
use crate::Rc;

pub struct Texture {
    handle:GLuint,
    image:ImageGL,
    options:TextureOptions
}

impl Texture {

    pub fn empty() -> Self {
        Self {
            handle: 0, image:ImageGL::empty(),
            options:TextureOptions::default()
        }
    }

    pub fn handle( &self ) -> &GLuint { &self.handle }
    pub fn width( &self )  -> &GLint  { &self.image.width }
    pub fn height( &self ) -> &GLint  { &self.image.height }
    pub fn image_data( &self ) -> &Vec<u8> { &self.image.data }
    pub fn wrapping_x( &self ) -> &TextureWrapping { &self.options.wrapping_x }
    pub fn wrapping_y( &self ) -> &TextureWrapping { &self.options.wrapping_y }
    pub fn min_filtering( &self ) -> &TextureFiltering { &self.options.min_filtering }
    pub fn mag_filtering( &self ) -> &TextureFiltering { &self.options.mag_filtering }

    pub fn use_texture( &self, sampler:&Sampler, location:GLint ) {
        unsafe {
            gl::ActiveTexture( sampler.handle() );
            gl::BindTexture( gl::TEXTURE_2D, *self.handle() );
            gl::Uniform1i( location, *sampler.id() );
        }
    }

    pub fn new( image:ImageGL, options:TextureOptions ) -> Rc<Self> {
        let mut handle = 0;

        unsafe {

            use fmath::types::color::RGB;

            gl::GenTextures( 1, &mut handle );
            gl::BindTexture( gl::TEXTURE_2D, handle );

            let mut use_border_color = false;
            let mut border_color = RGB::new_clear();

            let x_wrapping = match options.wrapping_x {
                TextureWrapping::Repeat         => gl::REPEAT as GLint,
                TextureWrapping::MirroredRepeat => gl::MIRRORED_REPEAT as GLint,
                TextureWrapping::ClampToEdge    => gl::CLAMP_TO_EDGE as GLint,
                TextureWrapping::ClampToBorder( color )  => {
                    border_color = color;
                    use_border_color = true;
                    gl::CLAMP_TO_BORDER as GLint
                },
            };

            let y_wrapping = match options.wrapping_y {
                TextureWrapping::Repeat         => gl::REPEAT as GLint,
                TextureWrapping::MirroredRepeat => gl::MIRRORED_REPEAT as GLint,
                TextureWrapping::ClampToEdge    => gl::CLAMP_TO_EDGE as GLint,
                TextureWrapping::ClampToBorder( color )  => {
                    border_color = color;
                    use_border_color = true;
                    gl::CLAMP_TO_BORDER as GLint
                },
            };

            if use_border_color {
                gl::TexParameterfv(
                    gl::TEXTURE_2D, gl::TEXTURE_BORDER_COLOR,
                    border_color.as_array_rgba_f32().as_ptr()
                );
            }

            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, x_wrapping );
            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, y_wrapping );

            let min_filter = match options.min_filtering {
                TextureFiltering::Nearest => gl::NEAREST as GLint,
                TextureFiltering::Linear => gl::LINEAR as GLint,
            };

            let mag_filter = match options.mag_filtering {
                TextureFiltering::Nearest => gl::NEAREST as GLint,
                TextureFiltering::Linear => gl::LINEAR as GLint,
            };

            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter );
            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter );

            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGB as GLint,
                image.width, image.height,
                0, image.format, gl::UNSIGNED_BYTE,
                image.data.as_ptr() as *const GLvoid
            );

            gl::GenerateMipmap( gl::TEXTURE_2D );

        }

        Rc::new( Texture { handle, image, options } )
    }

}

pub unsafe fn delete_textures( textures: Vec<Rc<Texture>> ) {

    let mut handles:Vec<GLuint> = Vec::with_capacity( textures.len() );
    for texture in textures.iter() {
        // make sure each texture being deleted is not in use
        assert!( Rc::strong_count( texture ) == 1 );
        handles.push( *texture.handle() );
    }
    drop( textures );
    gl::DeleteTextures( handles.len() as GLsizei, handles.as_ptr() );
}

pub struct Sampler { id:GLint }

impl Sampler {

    pub fn new( id:GLint ) -> Self { Self{ id } }
    pub fn empty() -> Self { Self{ id:0 } }

    pub fn handle( &self ) -> GLenum {
        gl::TEXTURE0 + (self.id as GLuint)
    }

    pub fn id( &self ) -> &GLint { &self.id }

}

impl core::fmt::Display for Sampler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "TEXTURE {}\n", self.id() )
    }
}

#[derive(Clone, Copy)]
pub struct TextureOptions {
    wrapping_x:TextureWrapping, wrapping_y:TextureWrapping,
    min_filtering:TextureFiltering, mag_filtering:TextureFiltering
}

impl TextureOptions {
    pub fn default() -> Self {
        Self {
            wrapping_x: TextureWrapping::Repeat, wrapping_y: TextureWrapping::Repeat,
            min_filtering: TextureFiltering::Nearest, mag_filtering: TextureFiltering::Linear,
        }
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

    pub fn set_filtering( &mut self, filtering:TextureFiltering ) {
        self.min_filtering = filtering;
        self.mag_filtering = filtering;
    }

    pub fn set_min_filtering( &mut self, filtering:TextureFiltering ) {
        self.min_filtering = filtering;
    }

    pub fn set_mag_filtering( &mut self, filtering:TextureFiltering ) {
        self.mag_filtering = filtering;
    }

}

#[derive(Clone, Copy)]
pub enum TextureWrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder( fmath::types::color::RGB ),
}

#[derive(Clone, Copy)]
pub enum TextureFiltering {
    Nearest,
    Linear,
}

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
            _ => return Err(
                Error::UnsupportedColorFormat( format!( "Load Texture Error: Unsupported Color Format!" ) )
            ),
        };

        Ok( 
            Self {
                width: dynamic.width() as GLint, height: dynamic.height() as GLint,
                format, data
            }
        )

    }

}

#[derive(Debug)]
pub enum Error {
    UnsupportedColorFormat(String),
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Error::UnsupportedColorFormat(s) => s,
        };
        write!( f, "{}", msg )
    }
}