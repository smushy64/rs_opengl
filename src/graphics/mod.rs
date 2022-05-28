mod vertex;
pub use vertex::Vertex;
mod uniform;
pub use uniform::Uniform;

pub mod camera;
pub use camera::Camera;

pub mod shader;
pub use shader::{ Shader, ShaderProgram };
pub mod material;
pub use material::Material;

pub mod mesh;
pub use mesh::Mesh;

pub mod texture;
pub use texture::{ Texture, Sampler };

use gl::types::*;
use core::fmt;

use crate::log;

pub fn load_glfn( video_subsystem:&sdl2::VideoSubsystem ) {
    gl::load_with(
        |symbol|
            video_subsystem.gl_get_proc_address(&symbol) as *const GLvoid
    );
}

pub fn set_clear_color( color:&fmath::types::color::RGB ) {
    let rgb = color.as_tuple_rgb_f32();
    unsafe { gl::ClearColor( rgb.0, rgb.1, rgb.2, 1.0 ); }
}

pub fn update_viewport( dimensions:&fmath::types::Vector2 ) {
    unsafe {
        gl::Viewport(
            0 as GLint, 0 as GLint,
            dimensions[0] as GLsizei, dimensions[1] as GLsizei
        );
    }
}

// TODO: Create clear screen mask abstraction
pub fn clear_screen( mask:u32 ) {
    unsafe { gl::Clear( gl::COLOR_BUFFER_BIT | mask ); }
}

// TODO: Finish enum definition!
#[derive(Debug, Clone, Copy)]
pub enum ScreenBuffers {
    Color   = 0x4000,
    Depth   = 0x100,
    Stencil = 0x400,
}

#[derive(Debug, Clone)]
pub struct Culling {
    enabled: bool,
    mode:    CullingMode,
    order:   WindingOrder,
}

impl Culling {
    pub fn initialize() -> Self {
        let mode  = CullingMode::Back;
        let order = WindingOrder::CounterClockwise;
        Self {
            enabled: true,
            mode, order
        }
    }

    pub fn initialize_disabled() -> Self {
        let mode  = CullingMode::Back;
        let order = WindingOrder::CounterClockwise;
        Self {
            enabled: false,
            mode, order
        }
    }

    pub fn is_enabled(&self) -> bool { self.enabled }
    pub fn update_gl( &self )   {
        Self::gl_enable( self.enabled );
        if self.enabled {
            Self::gl_set_winding_order( self.order as GLenum );
            Self::gl_set_culling_mode( self.mode as GLenum );
        }
    }
    pub fn enable( &mut self )  { self.enabled = true;  }
    pub fn disable( &mut self ) { self.enabled = false; }

    pub fn winding_order(&self) -> WindingOrder { self.order }
    pub fn set_winding_order( &mut self, order:WindingOrder ) { self.order = order; }

    pub fn culling_mode(&self)  -> CullingMode  { self.mode }
    pub fn set_culling_mode( &mut self, mode:CullingMode ) { self.mode = mode; }

    fn gl_set_culling_mode( g:GLenum ) {
        unsafe {
            gl::CullFace( g );
        }
    }

    fn gl_set_winding_order( g:GLenum ) {
        unsafe {
            gl::FrontFace( g );
        }
    }

    fn gl_enable( b:bool ) {
        unsafe {
            if b { gl::Enable( gl::CULL_FACE ) }
            else { gl::Disable( gl::CULL_FACE ) }
        }
    }
}

impl fmt::Display for Culling {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let enabled = if self.enabled { "Enabled" }
        else { "Disabled" };
        write!( f, "Culling {} | Winding Order: {} Mode: {}",
            enabled, self.winding_order(), self.culling_mode()
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum WindingOrder {
    Clockwise        = 0x900,
    CounterClockwise = 0x901,
}

impl TryFrom<GLenum> for WindingOrder {
    type Error = ();

    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            0x900 => Ok( Self::Clockwise        ),
            _     => Ok( Self::CounterClockwise ),
        }
    }
}

impl WindingOrder {
    pub fn as_glenum(&self) -> GLenum { *self as GLenum }
    pub fn from_glenum( g:GLenum ) -> Self { g.try_into().unwrap() }

    pub fn msg(&self) -> &str {
        match self {
            Self::Clockwise        => "Clockwise",
            Self::CounterClockwise => "Counter-Clockwise",
        }
    }
}

impl fmt::Display for WindingOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum CullingMode {
    Back         = 0x405,
    Front        = 0x404,
    FrontAndBack = 0x408,
}

impl TryFrom<GLenum> for CullingMode {
    type Error = ();

    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            0x404 => Ok( Self::Front        ),
            0x408 => Ok( Self::FrontAndBack ),
            _     => Ok( Self::Back         ),
        }
    }
}

impl CullingMode {
    pub fn as_glenum(&self) -> GLenum { *self as GLenum }
    pub fn from_glenum( g:GLenum ) -> Self { g.try_into().unwrap() }

    pub fn msg(&self) -> &str {
        match self {
            Self::Back         => "Back",
            Self::Front        => "Front",
            Self::FrontAndBack => "Front and Back",
        }
    }
}

impl fmt::Display for CullingMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}

/// TODO: sort meshes for rendering!
pub struct Blend {
    enabled: bool,
    rgb_source_factor       : BlendFactor,
    rgb_destination_factor  : BlendFactor,
    alpha_source_factor     : BlendFactor,
    alpha_destination_factor: BlendFactor,
    blend_mode              : BlendMode,
}

impl Blend {
    pub fn initialize() -> Self {
        Self::gl_enable( true );
        let src    = BlendFactor::One;
        let dst    = BlendFactor::Zero;
        let mode = BlendMode::Add;
        Self::gl_blend_func( src as GLenum, dst as GLenum );
        Self::gl_blend_equation( mode as GLenum );
        Self {
            enabled: true,
            rgb_source_factor:        src,
            rgb_destination_factor:   dst,
            alpha_source_factor:      src,
            alpha_destination_factor: dst,
            blend_mode:               mode,
        }
    }

    pub fn is_enabled(&self) -> bool { self.enabled }
    pub fn enable( &mut self )  { self.enabled = true;  Self::gl_enable(self.enabled) }
    pub fn disable( &mut self ) { self.enabled = false; Self::gl_enable(self.enabled) }

    pub fn src_factor_rgb( &self )   -> BlendFactor { self.rgb_source_factor        }
    pub fn dst_factor_rgb( &self )   -> BlendFactor { self.rgb_destination_factor   }
    pub fn src_factor_alpha( &self ) -> BlendFactor { self.alpha_source_factor      }
    pub fn dst_factor_alpha( &self ) -> BlendFactor { self.alpha_destination_factor }
    pub fn blend_mode( &self )       -> BlendMode   { self.blend_mode               }

    pub fn set_blend_mode( &mut self, mode:BlendMode ) {
        self.blend_mode = mode;
        Self::gl_blend_equation( mode as GLenum );
        log(
            &format!( "Set blend mode to \'{}\'.", mode ),
            "Blend"
        );
    }

    pub fn set_blend_factor( &mut self, src:BlendFactor, dst:BlendFactor ) {
        self.rgb_source_factor      = src;
        self.rgb_destination_factor = dst;
        self.alpha_source_factor      = src;
        self.alpha_destination_factor = dst;
        Self::gl_blend_func( src as GLenum, dst as GLenum );
        log(
            &format!(
                "Set src factor to \'{}\' and dst factor to \'{}\'",
                src, dst
            ),
            "Blend"
        );
    }

    pub fn set_blend_factor_separate( &mut self,
        src_rgb:BlendFactor, dst_rgb:BlendFactor,
        src_a:BlendFactor, dst_a:BlendFactor )
    {
        self.rgb_source_factor        = src_rgb;
        self.rgb_destination_factor   = dst_rgb;
        self.alpha_source_factor      = src_a;
        self.alpha_destination_factor = dst_a;
        Self::gl_blend_func_sep(
            src_rgb as GLenum, dst_rgb as GLenum,
            src_a   as GLenum, dst_a as GLenum
        );
        log(
            &format!(
                "Set src factor RGB to \'{}\' dst factor RGB to \'{}\' src factor Alpha to \'{}\' and dst factor Alpha to \'{}\'",
                src_rgb, dst_rgb, src_a, dst_a
            ),
            "Blend"
        );
    }

    fn gl_blend_equation( mode:GLenum ) {
        unsafe {
            gl::BlendEquation( mode );
        }
    }

    fn gl_blend_func( src:GLenum, dst:GLenum ) {
        unsafe {
            gl::BlendFunc( src, dst );
        }
    }

    fn gl_blend_func_sep( src_rgb:GLenum, dst_rgb:GLenum, src_a:GLenum, dst_a:GLenum ) {
        unsafe {
            gl::BlendFuncSeparate(
                src_rgb, dst_rgb,
                src_a, dst_a
            );
        }
    }

    fn gl_enable( b:bool ) {
        if b {
            unsafe{ gl::Enable( gl::BLEND ) }
            log(
                "Blending Enabled",
                "Blend"
            );
        }
        else {
            unsafe{ gl::Disable( gl::BLEND ) }
            log(
                "Blending Disabled",
                "Blend"
            );
        }
    }
}

impl fmt::Display for Blend {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f,
            "Src RGB Factor: \'{}\' Src Alpha Factor: \'{}\' Dst RGB Factor: \'{}\' Dst Alpha Factor: \'{}\' Mode: \'{}\'",
            self.src_factor_rgb(), self.src_factor_alpha(),
            self.dst_factor_rgb(), self.dst_factor_alpha(),
            self.blend_mode()
        )
    }
}

#[derive(Clone, Copy)]
pub enum BlendMode {
    Add             = 0x8006,
    Subtract        = 0x800A,
    ReverseSubtract = 0x800B,
    Min             = 0x8007,
    Max             = 0x8008,
}

impl TryFrom<GLenum> for BlendMode {
    type Error = ();

    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            0x800A => Ok( Self::Subtract        ),
            0x800B => Ok( Self::ReverseSubtract ),
            0x8007 => Ok( Self::Min             ),
            0x8008 => Ok( Self::Max             ),
            _      => Ok( Self::Add             ),
        }
    }
}

impl BlendMode {
    pub fn as_glenum(&self) -> GLenum { *self as GLenum }
    pub fn from_glenum( g:GLenum ) -> Self { g.try_into().unwrap() }

    pub fn msg(&self) -> &str {
        match self {
            Self::Add             => "Add",
            Self::Subtract        => "Subtract",
            Self::ReverseSubtract => "ReverseSubtract",
            Self::Min             => "Min",
            Self::Max             => "Max",
        }
    }
}

impl fmt::Display for BlendMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}

#[derive(Clone, Copy)]
pub enum BlendFactor {
    /// Factor is equal to 0
    Zero                  = 0     ,
    /// Factor is equal to 1
    One                   = 1     ,
    /// Factor is equal to the source color vector
    SrcColor              = 0x300 ,
    /// Factor is equal to 1 - source color vector
    OneMinusSrcColor      = 0x301 ,
    /// Factor is equal to the source alpha vector
    SrcAlpha              = 0x302 ,
    /// Factor is equal to 1 - source alpha vector
    OneMinusSrcAlpha      = 0x303 ,
    /// Factor is equal to the destination color vector
    DstColor              = 0x306 ,
    /// Factor is equal to 1 - destination color vector
    OneMinusDstColor      = 0x307 ,
    /// Factor is equal to the destination alpha vector
    DstAlpha              = 0x304 ,
    /// Factor is equal to 1 - destination alpha vector
    OneMinusDstAlpha      = 0x305 ,
    /// Factor is equal to the constant color vector
    ConstantColor         = 0x8001,
    /// Factor is equal to 1 - constant color vector
    OneMinusConstantColor = 0x8002,
    /// Factor is equal to the constant alpha vector
    ConstantAlpha         = 0x8003,
    /// Factor is equal to 1 - constant alpha vector
    OneMinusConstantAlpha = 0x8004,
}

impl TryFrom<GLenum> for BlendFactor {
    type Error = ();

    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            1      => Ok( Self::One                   ),
            0x300  => Ok( Self::SrcColor              ),
            0x301  => Ok( Self::OneMinusSrcColor      ),
            0x302  => Ok( Self::SrcAlpha              ),
            0x303  => Ok( Self::OneMinusSrcAlpha      ),
            0x304  => Ok( Self::DstAlpha              ),
            0x305  => Ok( Self::OneMinusDstAlpha      ),
            0x306  => Ok( Self::DstColor              ),
            0x307  => Ok( Self::OneMinusDstColor      ),
            0x8001 => Ok( Self::ConstantColor         ),
            0x8002 => Ok( Self::OneMinusConstantColor ),
            0x8003 => Ok( Self::ConstantAlpha         ),
            0x8004 => Ok( Self::OneMinusConstantAlpha ),
            _      => Ok( Self::Zero                  ),
        }
    }
}

impl BlendFactor {
    pub fn as_glenum(&self) -> GLenum { *self as GLenum }
    pub fn from_glenum( g:GLenum ) -> Self { g.try_into().unwrap() }

    pub fn msg(&self) -> &str {
        match self {
            Self::Zero                  => "Zero",
            Self::One                   => "One",
            Self::SrcColor              => "Src Color",
            Self::OneMinusSrcColor      => "One Minus Src Color",
            Self::SrcAlpha              => "Src Alpha",
            Self::OneMinusSrcAlpha      => "One Minus Src Alpha",
            Self::DstAlpha              => "Dst Alpha",
            Self::OneMinusDstAlpha      => "One Minus Dst Alpha",
            Self::DstColor              => "Dst Color",
            Self::OneMinusDstColor      => "One Minus Dst Color",
            Self::ConstantColor         => "Constant Color",
            Self::OneMinusConstantColor => "One Minus Constant Color",
            Self::ConstantAlpha         => "Constant Alpha",
            Self::OneMinusConstantAlpha => "One Minus Constant Alpha",
        }
    }
}

impl fmt::Display for BlendFactor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}

pub struct StencilTest {
    enabled: bool,
    mask:    GLuint,
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
            Self::Keep         => gl::KEEP      ,
            Self::Zero         => gl::ZERO      ,
            Self::Replace      => gl::REPLACE   ,
            Self::Increase     => gl::INCR      ,
            Self::IncreaseWrap => gl::INCR_WRAP ,
            Self::Decrease     => gl::DECR      ,
            Self::DecreaseWrap => gl::DECR_WRAP ,
            Self::Invert       => gl::INVERT    ,
        }
    }

    pub fn msg(&self) -> &str {
        match self {
            Self::Keep         => "Keep"          ,
            Self::Zero         => "Zero"          ,
            Self::Replace      => "Replace"       ,
            Self::Increase     => "Increase"      ,
            Self::IncreaseWrap => "Increase Wrap" ,
            Self::Decrease     => "Decrease"      ,
            Self::DecreaseWrap => "Decrease Wrap" ,
            Self::Invert       => "Invert"        ,
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
        let dt_info = if self.is_enabled() {
            format!( "Enabled: {}", self.depth_test_kind() )
        } else { format!("Disabled.") };
        write!( f, "Depth Test is {}", dt_info )
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
            Self::Always        => gl::ALWAYS   ,
            Self::Never         => gl::NEVER    ,
            Self::Less          => gl::LESS     ,
            Self::Equal         => gl::EQUAL    ,
            Self::LessEquals    => gl::LEQUAL   ,
            Self::Greater       => gl::GREATER  ,
            Self::GreaterEquals => gl::GEQUAL   ,
            Self::NotEqual      => gl::NOTEQUAL ,
        }
    }

    pub fn msg(&self) -> &str {
        match self {
            Self::Always        => "Always"         ,
            Self::Never         => "Never"          ,
            Self::Less          => "Less"           ,
            Self::Equal         => "Equal"          ,
            Self::LessEquals    => "Less Equals"    ,
            Self::Greater       => "Greater"        ,
            Self::GreaterEquals => "Greater Equals" ,
            Self::NotEqual      => "Not Equal"      ,
        }
    }
}

impl fmt::Display for TestKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}
