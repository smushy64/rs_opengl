mod material;
mod vertex;
mod model;

pub mod shader;
pub use shader::{ Shader, ShaderProgram, Error as ShaderError };

pub mod mesh;
pub use mesh::Mesh;

pub mod texture;
pub use texture::{ Texture, Sampler };

pub mod uniform;
pub use uniform::{ MaterialUniform, UniformValue };

pub use material::Material;
pub use vertex::Vertex;
pub use model::Model;

use gl::types::*;
use core::fmt;

pub fn load_glfn( subsys:&sdl2::VideoSubsystem ) {
    gl::load_with(
        |symbol|
            subsys.gl_get_proc_address(&symbol) as *const GLvoid
    );
}

pub fn clear_color( color:&fmath::types::color::RGB ) {
    let rgb = color.as_tuple_rgb_f32();
    unsafe {
        gl::ClearColor( rgb.0, rgb.1, rgb.2, 1.0 );
    }
}

pub fn clear_screen( mask:u32 ) {
    unsafe {
        gl::Clear( gl::COLOR_BUFFER_BIT | mask );
    }
}

pub fn set_viewport( dimensions:&fmath::types::Vector2 ) {
    unsafe {
        gl::Viewport(
            0 as GLint, 0 as GLint,
            dimensions[0] as GLsizei, dimensions[1] as GLsizei
        );
    }
}

pub struct StencilTest {
    enabled:   bool,
    mask:      GLuint,
}

impl StencilTest {
    pub fn initialize() -> Self {
        Self::gl_enable( true );
        Self::gl_stencil_mask( 0xFF );
        Self { enabled: true, mask: 0xFF }
    }

    pub fn is_enabled( &self ) -> bool { self.enabled }
    pub fn current_stencil_mask( &self ) -> GLuint { self.mask }

    pub fn func( &self, test:TestKind, reference:GLint, mask:GLuint ) {
        unsafe { gl::StencilFunc(test.as_glenum(), reference, mask) }
    }
    pub fn op( &self,
        stencil_fail:StencilAction,
        depth_fail:StencilAction,
        pass:StencilAction
    ) {
        unsafe {
            gl::StencilOp(
                stencil_fail.as_glenum(),
                depth_fail.as_glenum(),
                pass.as_glenum()
            );
        }
    }

    pub fn enable( &mut self ) {
        self.enabled = true;
        Self::gl_enable(self.enabled);
    }
    pub fn disable( &mut self ) {
        self.enabled = false;
        Self::gl_enable(self.enabled);
    }

    pub fn set_stencil_mask( &mut self, mask:GLuint ) {
        self.mask = mask;
        Self::gl_stencil_mask( mask );
    }

    fn gl_stencil_mask( mask:GLuint ) {
        unsafe { gl::StencilMask( mask ) }
    }

    fn gl_enable( b:bool ) {
        if b { unsafe{ gl::Enable( gl::STENCIL_TEST ) } }
        else { unsafe{ gl::Disable( gl::STENCIL_TEST ) } }
    }
}

#[derive(Clone, Copy)]
pub enum StencilAction {
    /// The currently stored stencil value is kept.
    Keep,
    /// The stencil value is set to 0.
    Zero,
    /// The stencil value is replaced with the reference value set with glStencilFunc.
    Replace,
    /// The stencil value is increased by 1 if it is lower than the maximum value.
    Increase,
    /// The stencil value is increased by 1, but wraps it back to 0 as soon as the maximum value is exceeded.
    IncreaseWrap,
    /// The stencil value is decreased by 1 if it is higher than the minimum value.
    Decrease,
    /// The stencil value is decreased by 1, but wraps it to the maximum value if it ends up lower than 0.
    DecreaseWrap,
    /// Bitwise inverts the current stencil buffer value.
    Invert
}

impl StencilAction {

    pub fn as_glenum( &self ) -> GLenum {
        match self {
            Self::Keep         => gl::KEEP     ,
            Self::Zero         => gl::ZERO     ,
            Self::Replace      => gl::REPLACE  ,
            Self::Increase     => gl::INCR     ,
            Self::IncreaseWrap => gl::INCR_WRAP,
            Self::Decrease     => gl::DECR     ,
            Self::DecreaseWrap => gl::DECR_WRAP,
            Self::Invert       => gl::INVERT   ,
        }
    }

    pub fn msg(&self) -> String {
        match self {
            Self::Keep         => format!( "Keep" ),
            Self::Zero         => format!( "Zero" ),
            Self::Replace      => format!( "Replace" ),
            Self::Increase     => format!( "Increase" ),
            Self::IncreaseWrap => format!( "Increase Wrap" ),
            Self::Decrease     => format!( "Decrease" ),
            Self::DecreaseWrap => format!( "Decrease Wrap" ),
            Self::Invert       => format!( "Invert" ),
        }
    }
}

impl fmt::Display for StencilAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}

pub struct DepthTest {
    enabled:   bool,
    test_kind: TestKind,
}

impl DepthTest {
    pub fn initialize() -> Self {
        let ( enabled, kind ) = ( true, TestKind::Less );
        Self::gl_enable( enabled );
        Self::gl_set_dt( &kind );
        Self { enabled, test_kind: kind }
    }

    pub fn is_enabled( &self )      -> bool      { self.enabled }
    pub fn depth_test_kind( &self ) -> &TestKind { &self.test_kind }

    pub fn enable( &mut self ) {
        self.enabled = true;
        Self::gl_enable(self.enabled);
    }
    pub fn disable( &mut self ) {
        self.enabled = false;
        Self::gl_enable(self.enabled);
    }
    pub fn set_depth_test_kind( &mut self, kind:TestKind ) {
        self.test_kind = kind;
        Self::gl_set_dt( &self.test_kind )
    }

    fn gl_enable( b:bool ) {
        if b { unsafe{ gl::Enable( gl::DEPTH_TEST ) } }
        else { unsafe{ gl::Disable( gl::DEPTH_TEST ) } }
    }

    fn gl_set_dt( dt:&TestKind ) {
        unsafe{ gl::DepthFunc( dt.as_glenum() ) }
    }

}

impl fmt::Display for DepthTest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let enabled = if self.is_enabled() {
            format!( "Enabled: {}", self.depth_test_kind() )
        } else { format!("Disabled.") };
        write!( f, "Depth Test is {}", enabled )
    }
}

#[derive(Clone, Copy)]
pub enum TestKind {
    Always,
    Never,
    Less,
    Equal,
    LessEquals,
    Greater,
    GreaterEquals,
    NotEqual,
}

impl TestKind {

    pub fn as_glenum( &self ) -> GLenum {
        match self {
            Self::Always        => gl::ALWAYS  ,
            Self::Never         => gl::NEVER   ,
            Self::Less          => gl::LESS    ,
            Self::Equal         => gl::EQUAL   ,
            Self::LessEquals    => gl::LEQUAL  ,
            Self::Greater       => gl::GREATER ,
            Self::GreaterEquals => gl::GEQUAL  ,
            Self::NotEqual      => gl::NOTEQUAL,
        }
    }

    pub fn msg(&self) -> String {
        match self {
            Self::Always        => format!( "Always" ),
            Self::Never         => format!( "Never" ),
            Self::Less          => format!( "Less" ),
            Self::Equal         => format!( "Equal" ),
            Self::LessEquals    => format!( "Less Equals" ),
            Self::Greater       => format!( "Greater" ),
            Self::GreaterEquals => format!( "Greater Equals" ),
            Self::NotEqual      => format!( "Not Equal" ),
        }
    }
}

impl fmt::Display for TestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}
