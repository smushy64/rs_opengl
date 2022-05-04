use gl::types::*;

use super::c_string;

pub fn gl_error_compilation( id:GLuint ) -> String {
    let mut len:GLint = 0;
    unsafe {

        gl::GetShaderiv(
            id, gl::INFO_LOG_LENGTH,
            &mut len
        );

        let message = c_string::create_empty_c_string( len as usize );
        gl::GetShaderInfoLog(
            id, len,
            core::ptr::null_mut(),
            message.as_ptr() as *mut GLchar
        );

        return message.to_string_lossy().into_owned();

    }
}

pub fn gl_error_linking( id:GLuint ) -> String {
    let mut len:GLint = 0;
    unsafe {

        gl::GetProgramiv(
            id, gl::INFO_LOG_LENGTH,
            &mut len
        );

        let message = c_string::create_empty_c_string( len as usize );
        gl::GetProgramInfoLog(
            id, len,
            core::ptr::null_mut(),
            message.as_ptr() as *mut GLchar
        );

        return message.to_string_lossy().into_owned();

    }
}