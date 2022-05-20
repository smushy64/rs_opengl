use gl::types::*;

pub struct Texture {
    handle:GLuint,
    coord:TexCoord,
}

impl Texture {

    pub fn handle( &self ) -> &GLuint {
        &self.handle
    }

    pub fn set_texcoord( &mut self, new_coord:TexCoord ) {
        self.coord = new_coord;
    }

    pub fn texcoord( &self ) -> &TexCoord {
        &self.coord
    }

    pub fn use_texture( &self ) {
        unsafe {
            gl::ActiveTexture( self.texcoord().get_handle() );
            gl::BindTexture( gl::TEXTURE_2D, *self.handle() );
        }
    }

    pub fn new( image: ImageGL ) -> TextureBuilder {
        TextureBuilder::default( image )
    }

}

pub fn delete_textures( textures: Vec<Texture> ) {

    let mut handles:Vec<GLuint> = Vec::with_capacity( textures.len() );
    for texture in textures {
        handles.push( texture.handle );
    }

    unsafe {
        gl::DeleteTextures( handles.len() as GLsizei, handles.as_ptr() );
    }

}

pub enum TexCoord {
    ID( GLint )
}

impl TexCoord {
    pub fn get_handle( &self ) -> GLenum {
        match self {
            TexCoord::ID( coord ) => gl::TEXTURE0 + (*coord as GLuint),
        }
    }

    pub fn get_id( &self ) -> &GLint {
        match self {
            TexCoord::ID( id ) => id,
        }
    }
}

impl core::fmt::Display for TexCoord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!( f, "TEXTURE {}\n", self.get_handle() - gl::TEXTURE0 )
    }
}

pub struct TextureBuilder {
    image: ImageGL,
    x_wrapping: TextureWrapping, y_wrapping: TextureWrapping,
    minimizing_filter: TextureFiltering, magnifying_filter: TextureFiltering,
    coord: TexCoord
}

impl TextureBuilder {

    fn default( image:ImageGL ) -> Self {

        TextureBuilder { 

            image,

            x_wrapping: TextureWrapping::Repeat,
            y_wrapping: TextureWrapping::Repeat,

            minimizing_filter: TextureFiltering::Nearest,
            magnifying_filter: TextureFiltering::Nearest,

            coord: TexCoord::ID(0),

        }

    }

    pub fn set_linear_filtering( mut self ) -> Self {
        self.minimizing_filter = TextureFiltering::Linear;
        self.magnifying_filter = TextureFiltering::Linear;
        self
    }

    pub fn set_x_wrapping( mut self, wrapping:TextureWrapping ) -> Self {
        self.x_wrapping = wrapping;
        self
    }

    pub fn set_y_wrapping( mut self, wrapping:TextureWrapping ) -> Self {
        self.y_wrapping = wrapping;
        self
    }

    pub fn set_minimizing_filter( mut self, filter:TextureFiltering ) -> Self {
        self.minimizing_filter = filter;
        self
    }

    pub fn set_magnifying_filter( mut self, filter:TextureFiltering ) -> Self {
        self.magnifying_filter = filter;
        self
    }

    pub fn set_texcoord( mut self, coord:TexCoord ) -> Self {
        self.coord = coord;
        self
    }

    pub fn build( self ) -> Texture {

        let mut handle = 0;

        unsafe {

            use fmath::types::color::RGB;

            gl::GenTextures( 1, &mut handle );
            gl::BindTexture( gl::TEXTURE_2D, handle );

            let mut use_border_color = false;
            let mut border_color = RGB::new_clear();

            let x_wrapping = match self.x_wrapping {
                TextureWrapping::Repeat         => gl::REPEAT as GLint,
                TextureWrapping::MirroredRepeat => gl::MIRRORED_REPEAT as GLint,
                TextureWrapping::ClampToEdge    => gl::CLAMP_TO_EDGE as GLint,
                TextureWrapping::ClampToBorder( color )  => {
                    border_color = color;
                    use_border_color = true;
                    gl::CLAMP_TO_BORDER as GLint
                },
            };

            let y_wrapping = match self.y_wrapping {
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
                    border_color.as_float_rgba_array().as_ptr()
                );
            }

            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, x_wrapping );
            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, y_wrapping );

            let min_filter = match self.minimizing_filter {
                TextureFiltering::Nearest => gl::NEAREST as GLint,
                TextureFiltering::Linear => gl::LINEAR as GLint,
            };

            let mag_filter = match self.magnifying_filter {
                TextureFiltering::Nearest => gl::NEAREST as GLint,
                TextureFiltering::Linear => gl::LINEAR as GLint,
            };

            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, min_filter );
            gl::TexParameteri( gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mag_filter );

            gl::TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGB as GLint,
                self.image.width, self.image.height,
                0, self.image.format, gl::UNSIGNED_BYTE,
                self.image.data.as_ptr() as *const GLvoid
            );

            gl::GenerateMipmap( gl::TEXTURE_2D );

        }

        Texture {
            handle, coord: self.coord,
        }

    }

}

pub enum TextureWrapping {
    Repeat,
    MirroredRepeat,
    ClampToEdge,
    ClampToBorder( fmath::types::color::RGB ),
}

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

    pub fn from_dynamic_image( dynamic_image: image::DynamicImage ) -> Result<Self, Error> {

        let ( format, data) = match dynamic_image.color() {
            image::ColorType::Rgb8  => ( gl::RGB,  dynamic_image.to_rgb8().as_raw().clone()  ),
            image::ColorType::Rgba8 => ( gl::RGBA, dynamic_image.to_rgba8().as_raw().clone() ),
            _ => return Err( Error::UnsupportedColorType ),
        };

        Ok( 
            Self {
                width: dynamic_image.width() as GLint,
                height: dynamic_image.height() as GLint,
                format, data
            }
        )

    }

}

#[derive(Debug)]
pub enum Error {
    UnsupportedColorType,
}