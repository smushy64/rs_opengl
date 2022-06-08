extern crate colored;
pub use colored::Colorize;
use core::fmt;
use gl::types::*;
use crate::cstr;

pub fn log( _msg:&str, _src:&str ) {
    #[cfg(debug_assertions)]
    unsafe {
        if !SEVERITY._matches_svr( GLDebugSeverity::Info ) { return; }
        println!( "{} | Log\n   Source: {}\n   \"{}\"\n",
            "Engine".bold(),
            _src.bold(), _msg.white()
        );
    }
}

pub enum Error {

    ResourcesReadFile(String)           ,
    ResourcesNoFileType(String)         ,
    ResourcesUnrecognizedFileExt(String),
    
    ImageCrateLoad(String),

    OBJParse(String),
    GLTFJsonError(String),

    TextureUnsupportedColorFormat,

    ShaderLinker(String)   ,
    ShaderCompiler(String) ,
    ShaderParse(String)    ,

    UniformNotFound(String),

    CStringNul(String)     ,
    UTF8(String)           ,
    ParseFloat(String)     ,

}

impl Error {

    pub fn src(&self) -> &str {
        match self {

            Self::ImageCrateLoad(_)               => "image Crate",

            Self::ResourcesReadFile(_)            |
            Self::ResourcesNoFileType(_)          |
            Self::ResourcesUnrecognizedFileExt(_) => "Resources",

            Self::OBJParse(_) => "Wavefront OBJ Parser",
            Self::GLTFJsonError(_) => "glTF Parser",

            Self::TextureUnsupportedColorFormat => "Texture",

            Self::ShaderLinker(_)   => "Shader Linker",
            Self::ShaderCompiler(_) => "Shader Compiler",
            Self::ShaderParse(_)    => "Shader Parsing",

            Self::UniformNotFound(_) => "Uniform Values",

            Self::CStringNul(_)     => "CString Null",
            Self::UTF8(_)           => "UTF-8 Conversion",

            Self::ParseFloat(_) => "Float Parse from String",
        }
    }

    pub fn msg(&self) -> &str {
        match self {

            Self::TextureUnsupportedColorFormat => "Unsupported Color Format!",
            Self::ImageCrateLoad(s)               |
            Self::ResourcesReadFile(s)            |
            Self::ResourcesNoFileType(s)          |
            Self::ResourcesUnrecognizedFileExt(s) |
            Self::OBJParse(s)                     |
            Self::GLTFJsonError(s)                |
            Self::ShaderLinker(s)                 |
            Self::ShaderCompiler(s)               |
            Self::ShaderParse(s)                  |
            Self::UniformNotFound(s)              |
            Self::CStringNul(s)                   |
            Self::UTF8(s)                         |
            Self::ParseFloat(s)
            => s,
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "\nEngine | {}\n   Source: {}\n   \"{}\"\n",
            "Fatal Error".red().bold(),
            self.src(), self.msg().red().bold()
        )
    }
}

pub fn print_start() {
    let (major, minor) = version_str();
    let mode = if cfg!(debug_assertions) {
        "Debug Mode".yellow().bold()
    } else {
        "Release Mode".green().bold()
    };
    let version =
        format!("Using {}", format!("OpenGL {}.{}", major, minor).bold() );
    println!(
        "Learn OpenGL: {}\n   {}\n   Vendor: {}\n   Renderer: {}\n",
        mode, version,
        gl_str( &gl::VENDOR   ).bold(),
        gl_str( &gl::RENDERER ).bold()
    );
}

pub fn version_str() -> ( GLint, GLint ) {
    let mut major = 0;
    let mut minor = 0;
    unsafe {
        gl::GetIntegerv( gl::MAJOR_VERSION, &mut major );
        gl::GetIntegerv( gl::MINOR_VERSION, &mut minor );
    }
    ( major, minor )
}

pub fn gl_str( name:&GLenum ) -> &str {
    unsafe {
        use cstr::CStr;
        let ptr = gl::GetString( *name );
        CStr::from_ptr( ptr as *const i8 ).to_str().unwrap()
    }
}

pub fn set_severity( _with_severity:GLDebugSeverity ) {
    #[cfg(debug_assertions)]
    unsafe {
        SEVERITY = _with_severity;
        gl::Enable(gl::DEBUG_OUTPUT);
        gl::DebugMessageCallback(
            Some(message_callback as MsgCallback),
            core::ptr::null() as *const GLvoid
        );
    }
}

#[cfg(debug_assertions)]
pub type MsgCallback = extern "system" fn(
    src:        GLenum ,    type_:    GLenum,
    id:         GLuint ,    severity: GLenum,
    _len:       GLsizei,    msg:      *const GLchar,
    _usr_param: *mut GLvoid
);

#[cfg(debug_assertions)]
extern "system" fn message_callback(
    src:        GLenum ,    type_:    GLenum,
    id:         GLuint ,    severity: GLenum,
    _len:       GLsizei,    msg:      *const GLchar,
    _usr_param: *mut GLvoid
)
{
    let source = match src {
        gl::DEBUG_SOURCE_API             => "API"             ,
        gl::DEBUG_SOURCE_WINDOW_SYSTEM   => "WINDOW SYSTEM"   ,
        gl::DEBUG_SOURCE_SHADER_COMPILER => "SHADER COMPILER" ,
        gl::DEBUG_SOURCE_THIRD_PARTY     => "THIRD PARTY"     ,
        gl::DEBUG_SOURCE_APPLICATION     => "APPLICATION"     ,
        _                                => "UNKNOWN"         ,
    };

    let _type = match type_ {
        gl::DEBUG_TYPE_ERROR               => "ERROR"               ,
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => "DEPRECATED BEHAVIOR" ,
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR  => "UNDEFINED BEHAVIOUR" ,
        gl::DEBUG_TYPE_PORTABILITY         => "PORTABILITY"         ,
        gl::DEBUG_TYPE_PERFORMANCE         => "PERFORMANCE"         ,
        gl::DEBUG_TYPE_MARKER              => "MARKER"              ,
        gl::DEBUG_TYPE_PUSH_GROUP          => "PUSH GROUP"          ,
        gl::DEBUG_TYPE_POP_GROUP           => "POP GROUP"           ,
        gl::DEBUG_TYPE_OTHER               => "OTHER"               ,
        _                                  => "UNKNOWN"             ,
    };

    let svr = GLDebugSeverity::_from_glenum( severity );
    unsafe { if !SEVERITY._matches_svr( svr ) { return; } }

    use cstr::CStr;
    let message = unsafe { CStr::from_ptr( msg ).to_str().unwrap() };

    println!(
        "{} | ID: {:6} |\n   Severity: {}\n   Type:     {}\n   Source:   {}\n   \"{}\"\n",
        "OpenGL".bold(),
        id, svr, _type, source, message.white()
    );
}

#[cfg(debug_assertions)]
static mut SEVERITY:GLDebugSeverity = GLDebugSeverity::Medium;

#[derive(Clone, Copy)]
pub enum GLDebugSeverity {
    High    = 0x9146,
    Medium  = 0x9147,
    Low     = 0x9148,
    Info    = 0x826B,
    Unknown = 0,
}

impl TryFrom<GLenum> for GLDebugSeverity {
    type Error = ();

    fn try_from(value: GLenum) -> Result<Self, Self::Error> {
        match value {
            gl::DEBUG_SEVERITY_HIGH         => Ok( GLDebugSeverity::High    ),
            gl::DEBUG_SEVERITY_MEDIUM       => Ok( GLDebugSeverity::Medium  ),
            gl::DEBUG_SEVERITY_LOW          => Ok( GLDebugSeverity::Low     ),
            gl::DEBUG_SEVERITY_NOTIFICATION => Ok( GLDebugSeverity::Info    ),
            _                               => Ok( GLDebugSeverity::Unknown ),
        }
    }
}

impl GLDebugSeverity {

    fn _from_glenum( g:GLenum ) -> Self { g.try_into().unwrap() }

    fn _msg(&self) -> colored::ColoredString {
        let result = match self {
            GLDebugSeverity::High   => "HIGH"   .red(),
            GLDebugSeverity::Medium => "MEDIUM" .truecolor( 255, 165, 0 ), // orange
            GLDebugSeverity::Low    => "LOW"    .yellow(),
            GLDebugSeverity::Info   => "INFO"   .white(),
            _                => "UNKNOWN".green(),
        };
        result.bold()
    }

    fn _matches_svr(&self, other:Self ) -> bool {
        match self {
            GLDebugSeverity::High => {
                match other {
                    GLDebugSeverity::High => true,
                    _ => false,
                }
            },
            GLDebugSeverity::Medium => {
                match other {
                    GLDebugSeverity::High    => true,
                    GLDebugSeverity::Medium  => true,
                    GLDebugSeverity::Unknown => true,
                    _ => false,
                }
            },
            GLDebugSeverity::Low => {
                match other {
                    GLDebugSeverity::High    => true,
                    GLDebugSeverity::Medium  => true,
                    GLDebugSeverity::Low     => true,
                    GLDebugSeverity::Unknown => true,
                    _ => false,
                }
            },
            _ => true,
        }
    }
}

impl core::fmt::Display for GLDebugSeverity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!( f, "{}", self._msg() )
    }
}