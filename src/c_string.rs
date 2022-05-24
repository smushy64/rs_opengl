pub use std::ffi::{ CStr, CString };
use core::fmt;

pub fn create_empty_c_string( len:usize ) -> CString {
    let mut buffer:Vec<u8> = Vec::with_capacity( len + 1 );
    buffer.extend(
        [b' '].iter().cycle().take( len as usize )
    );
    unsafe { CString::from_vec_unchecked( buffer ) }
}

pub fn c_string_from_str( text:&str ) -> Result< CString, Error > {
    Ok( CString::new(text)
        .map_err( |e| Error::CStringToStr(format!("{}", e) ) )?
    )
}

pub fn format_c_string_uniform_name( name:CString ) -> Result< CString, Error > {
    let name_string = name.to_string_lossy().into_owned();
    c_string_from_str( name_string.trim_end().trim_end_matches( char::from(0) ) )
}

pub fn to_string( c_string:CString ) -> String {
    String::from(
        c_string.to_string_lossy()
            .trim_end()
            .trim_end_matches( char::from( 0 ) )
    )
}

#[derive(Debug)]
pub enum Error {
    CStringToStr(String),
}

impl Error {
    pub fn msg(&self) -> String {
        match self {
            Error::CStringToStr(s) => s.clone(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!( f, "{}", self.msg() )
    }
}
