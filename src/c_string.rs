pub use std::ffi::{ CStr, CString };

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

#[derive(Debug)]
pub enum Error {
    CStringToStr(String),
}