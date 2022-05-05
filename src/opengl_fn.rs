use gl::types::*;
use fmath::types::*;

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

pub fn ortho(
    left:f32, right:f32,
    bottom: f32, top:f32,
    near:f32, far:f32,
) -> Matrix4x4 {
    Matrix4x4::from_array([
        2.0 / ( right - left ), 0.0, 0.0, -(( right + left ) / ( right - left )),
        0.0, 2.0 / ( top - bottom ), 0.0, -(( top + bottom ) / ( top - bottom )),
        0.0, 0.0, -2.0 / ( far - near ), -(( far + near ) / ( far - near )),
        0.0, 0.0, 0.0, 1.0,
    ])
}

pub fn persp(
    fov_rad:f32,
    aspect:f32,
    near:f32, far:f32
) -> Matrix4x4 {

    let a = 1.0 / ( aspect * ( fov_rad / 2.0 ).tan() );
    let b = 1.0 / ( ( fov_rad / 2.0 ).tan() );
    let c = -(( far + near ) / ( far - near ));
    let d = -(( 2.0 * far * near ) / ( far - near ));

    Matrix4x4::from_array([
        a, 0.0, 0.0,  0.0,
      0.0,   b, 0.0,  0.0,
      0.0, 0.0,   c, -1.0,
      0.0, 0.0,   d,  0.0,
  ])
}