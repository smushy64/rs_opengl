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
    top: f32, bottom:f32,
    near:f32, far:f32,
) -> Matrix4x4 {

    let x = (
        2.0 / ( right - left ),
        -(( right + left ) / ( right - left ))
    );

    let y = (
        2.0 / ( top - bottom ),
        -(( top + bottom ) / ( top - bottom ))
    );

    let z = (
        -2.0 / ( far - near ),
        -((far + near) / (far - near))
    );

    Matrix4x4::from_array([
        x.0, 0.0, 0.0, 0.0,
        0.0, y.0, 0.0, 0.0,
        0.0, 0.0, z.0, 0.0,
        x.1, y.1, z.1, 1.0,
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