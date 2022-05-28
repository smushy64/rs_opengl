pub use std::ffi::{ CStr, CString };

pub trait CStringExt {
    fn whitespace_buffer( len:usize ) -> CString;
}

impl CStringExt for CString {
    fn whitespace_buffer( len:usize ) -> CString {
        let mut buffer:Vec<u8> = Vec::with_capacity( len + 1 );
        buffer.extend( [b' '].iter().cycle().take( len as usize ) );
        unsafe { CString::from_vec_unchecked( buffer ) }
    }
}
