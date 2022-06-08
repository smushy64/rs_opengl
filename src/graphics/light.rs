use fmath::types::*;
// use gl::types::*;

unsafe fn any_as_u8_slice<T: Sized>( s:&T ) -> &[u8] {
    core::slice::from_raw_parts(
        (s as *const T) as *const u8,
        core::mem::size_of::<T>()
    )
}

#[repr(C)]
pub struct Lights {
    pub directional_light: DirectionalLight,
    pub point_lights:      [PointLight;4],
    pub spot_lights:       [SpotLight;2],
}

impl Lights {
    pub fn new() -> Self {
        Self {
            directional_light:  DirectionalLight::default(),
            point_lights: [PointLight::default();4],
            spot_lights:  [SpotLight::default();2],
        }
    }

    pub fn directional_light_offset(&self) -> usize { 0 }
    pub fn point_light_offset(&self, light_idx:usize) -> usize {
        self.directional_light.size() + ( self.point_lights[0].size() * light_idx )
    }
    pub fn spot_light_offset(&self, light_idx:usize) -> usize {
        self.directional_light.size() +
        ( self.point_lights[0].size() * self.point_lights.len() ) +
        ( self.spot_lights[0].size() * light_idx )
    }

    pub fn active_point_light_count(&self) -> u32 {
        let mut buffer = 0;
        for point_light in self.point_lights.iter() {
            if point_light.is_active() { buffer += 1; }
        }
        buffer
    }

    pub fn active_spot_light_count(&self) -> u32 {
        let mut buffer = 0;
        for spot_light in self.spot_lights.iter() {
            if spot_light.is_active() { buffer += 1; }
        }
        buffer
    }

    pub fn active_light_count(&self) -> u32 {
        self.active_point_light_count() + self.active_spot_light_count()
    }

    pub fn as_bytes(&self) -> &[u8] { unsafe { any_as_u8_slice( self ) } }

    pub fn size(&self) -> usize { core::mem::size_of::<Self>() }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DirectionalLight {
    direction: Vector4,
    diffuse:   Vector4,
    specular:  Vector4,
}

const DIRECTIONAL_DIRECTION_BYTE_OFFSET:usize = 0;
const DIRECTIONAL_DIFFUSE_BYTE_OFFSET:usize   = 16;
const DIRECTIONAL_SPECULAR_BYTE_OFFSET:usize  = 32;

impl DirectionalLight {
    pub fn new( direction:Vector3, diffuse:color::RGB, specular:color::RGB ) -> Self {
        Self {
            direction: direction.as_vector4(),
            diffuse:   diffuse.as_vector4(),
            specular:  specular.as_vector4()
        }
    }

    pub fn default() -> Self {
        Self {
            direction: Vector3::new_up().as_vector4(),
            diffuse:   color::RGB::new_white().as_vector4(),
            specular:  color::RGB::new_white().as_vector4(),
        }
    }

    pub fn direction_offset(&self) -> usize { DIRECTIONAL_DIRECTION_BYTE_OFFSET }
    pub fn direction(&self) -> Vector3 { self.direction.as_vector3() }
    pub fn set_direction( &mut self, direction:Vector3 ) {
        self.direction = (-direction).as_vector4();
    }

    pub fn diffuse_offset(&self) -> usize { DIRECTIONAL_DIFFUSE_BYTE_OFFSET }
    pub fn diffuse(&self) -> color::RGB { color::RGB::from_array_rgba_f32( self.diffuse.as_array().clone() ) }
    pub fn set_diffuse( &mut self, diffuse:color::RGB ) {
        self.diffuse = diffuse.as_vector4();
    }

    pub fn specular_offset(&self) -> usize { DIRECTIONAL_SPECULAR_BYTE_OFFSET }
    pub fn specular(&self) -> color::RGB { color::RGB::from_array_rgba_f32( self.specular.as_array().clone() ) }
    pub fn set_specular( &mut self, specular:color::RGB ) {
        self.specular = specular.as_vector4();
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            any_as_u8_slice( self )
        }
    }

    pub fn size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PointLight {
    position:  Vector4,
    diffuse:   Vector4,
    specular:  Vector4,
    constant:  f32,
    linear:    f32,
    quadratic: f32,

    active: u32,
}

const POINT_POSITION_BYTE_OFFSET:usize  = 0;
const POINT_DIFFUSE_BYTE_OFFSET:usize   = 16;
const POINT_SPECULAR_BYTE_OFFSET:usize  = 32;
const POINT_CONSTANT_BYTE_OFFSET:usize  = 48;
const POINT_LINEAR_BYTE_OFFSET:usize    = 52;
const POINT_QUADRATIC_BYTE_OFFSET:usize = 56;
const POINT_ACTIVE_BYTE_OFFSET:usize    = 60;

impl PointLight {
    pub fn new(
        position:Vector3, diffuse:color::RGB, specular:color::RGB,
        constant:f32, linear:f32, quadratic:f32, active: bool
    ) -> Self {
        Self {
            position: position.as_vector4(),
            diffuse:   diffuse.as_vector4(),
            specular: specular.as_vector4(),
            constant, linear, quadratic,
            active: if active { 1 } else { 0 }
        }
    }

    pub fn default() -> Self {
        Self {
            position: Vector3::new_zero().as_vector4(),
            diffuse:  color::RGB::new_white().as_vector4(),
            specular: color::RGB::new_white().as_vector4(),
            constant: 1.0, linear: 0.14, quadratic: 0.07,
            active: 0
        }
    }

    pub fn position_offset(&self) -> usize { POINT_POSITION_BYTE_OFFSET }
    pub fn position(&self) -> Vector3 { self.position.as_vector3() }
    pub fn set_position( &mut self, position:Vector3 ) {
        self.position = position.as_vector4();
    }

    pub fn diffuse_offset(&self) -> usize { POINT_DIFFUSE_BYTE_OFFSET }
    pub fn diffuse(&self) -> color::RGB { color::RGB::from_array_rgba_f32( self.diffuse.as_array().clone() ) }
    pub fn set_diffuse( &mut self, diffuse:color::RGB ) {
        self.diffuse = diffuse.as_vector4();
    }

    pub fn specular_offset(&self) -> usize { POINT_SPECULAR_BYTE_OFFSET }
    pub fn specular(&self) -> color::RGB { color::RGB::from_array_rgba_f32( self.specular.as_array().clone() ) }
    pub fn set_specular( &mut self, specular:color::RGB ) {
        self.specular = specular.as_vector4();
    }

    pub fn constant_offset(&self) -> usize { POINT_CONSTANT_BYTE_OFFSET }
    pub fn constant(&self) -> f32 { self.constant }
    pub fn set_constant( &mut self, constant:f32 ) {
        self.constant = constant;
    }

    pub fn linear_offset(&self) -> usize { POINT_LINEAR_BYTE_OFFSET }
    pub fn linear(&self) -> f32 { self.linear }
    pub fn set_linear( &mut self, linear:f32 ) {
        self.linear = linear;
    }

    pub fn quadratic_offset(&self) -> usize { POINT_QUADRATIC_BYTE_OFFSET }
    pub fn quadratic(&self) -> f32 { self.quadratic }
    pub fn set_quadratic( &mut self, quadratic:f32 ) {
        self.quadratic = quadratic;
    }

    pub fn active_bytes(&self) -> [u8;4] { self.active.to_le_bytes() }
    pub fn active_offset(&self) -> usize { POINT_ACTIVE_BYTE_OFFSET }
    pub fn is_active(&self) -> bool { self.active != 0 }
    pub fn set_active( &mut self, active:bool ) {
        self.active = if active { 1 } else { 0 }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            any_as_u8_slice( self )
        }
    }

    pub fn size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SpotLight {
    position:  Vector4,
    direction: Vector4,
    diffuse:   Vector4,
    specular:  Vector4,

    inner_cutoff: f32,
    outer_cutoff: f32,

    constant:  f32,
    linear:    f32,
    quadratic: f32,

    active: u32,

    padding: [u32;2],
}

const SPOT_POSITION_BYTE_OFFSET:usize  = 0;
const SPOT_DIRECTION_BYTE_OFFSET:usize = 16;
const SPOT_DIFFUSE_BYTE_OFFSET:usize   = 32;
const SPOT_SPECULAR_BYTE_OFFSET:usize  = 48;
const SPOT_INNER_BYTE_OFFSET:usize     = 64;
const SPOT_OUTER_BYTE_OFFSET:usize     = 68;
const SPOT_CONSTANT_BYTE_OFFSET:usize  = 72;
const SPOT_LINEAR_BYTE_OFFSET:usize    = 76;
const SPOT_QUADRATIC_BYTE_OFFSET:usize = 80;
const SPOT_ACTIVE_BYTE_OFFSET:usize    = 84;

impl SpotLight {
    pub fn new(
        position:Vector3, direction:Vector3,
        diffuse:color::RGB, specular:color::RGB,
        inner_cutoff:f32, outer_cutoff:f32,
        constant:f32, linear:f32, quadratic:f32,
        active: bool
    ) -> Self {
        Self {
            position:  position.as_vector4(),
            direction: direction.as_vector4(),

            diffuse:   diffuse.as_vector4(),
            specular: specular.as_vector4(),

            inner_cutoff, outer_cutoff,

            constant, linear, quadratic,
            active: if active { 1 } else { 0 },

            padding: [0u32;2],
        }
    }

    pub fn default() -> Self {
        Self {
            position:  Vector3::new_zero().as_vector4(),
            direction: Vector3::new_down().as_vector4(),
            diffuse:  color::RGB::new_white().as_vector4(),
            specular: color::RGB::new_white().as_vector4(),
            inner_cutoff: 25.0f32.to_radians().cos(),
            outer_cutoff: 28.0f32.to_radians().cos(),
            constant: 1.0, linear: 0.14, quadratic: 0.07,
            active: 0,
            padding: [0u32;2],
        }
    }

    pub fn position_offset(&self) -> usize { SPOT_POSITION_BYTE_OFFSET }
    pub fn position(&self) -> Vector3 { self.position.as_vector3() }
    pub fn set_position( &mut self, position:Vector3 ) {
        self.position = position.as_vector4();
    }

    pub fn direction_offset(&self) -> usize { SPOT_DIRECTION_BYTE_OFFSET }
    pub fn direction(&self) -> Vector3 { self.direction.as_vector3() }
    pub fn set_direction( &mut self, direction:Vector3 ) {
        self.direction = direction.as_vector4();
    }

    pub fn diffuse_offset(&self) -> usize { SPOT_DIFFUSE_BYTE_OFFSET }
    pub fn diffuse(&self) -> color::RGB { color::RGB::from_array_rgba_f32( self.diffuse.as_array().clone() ) }
    pub fn set_diffuse( &mut self, diffuse:color::RGB ) {
        self.diffuse = diffuse.as_vector4();
    }

    pub fn specular_offset(&self) -> usize { SPOT_SPECULAR_BYTE_OFFSET }
    pub fn specular(&self) -> color::RGB { color::RGB::from_array_rgba_f32( self.specular.as_array().clone() ) }
    pub fn set_specular( &mut self, specular:color::RGB ) {
        self.specular = specular.as_vector4();
    }

    pub fn inner_cutoff_offset(&self) -> usize { SPOT_INNER_BYTE_OFFSET }
    pub fn inner_cutoff(&self) -> f32 { self.inner_cutoff }
    pub fn set_inner_cutoff( &mut self, inner_cutoff:f32 ) {
        self.inner_cutoff = inner_cutoff;
    }

    pub fn outer_cutoff_offset(&self) -> usize { SPOT_OUTER_BYTE_OFFSET }
    pub fn outer_cutoff(&self) -> f32 { self.outer_cutoff }
    pub fn set_outer_cutoff( &mut self, outer_cutoff:f32 ) {
        self.outer_cutoff = outer_cutoff;
    }

    pub fn constant_offset(&self) -> usize { SPOT_CONSTANT_BYTE_OFFSET }
    pub fn constant(&self) -> f32 { self.constant }
    pub fn set_constant( &mut self, constant:f32 ) {
        self.constant = constant;
    }

    pub fn linear_offset(&self) -> usize { SPOT_LINEAR_BYTE_OFFSET }
    pub fn linear(&self) -> f32 { self.linear }
    pub fn set_linear( &mut self, linear:f32 ) {
        self.linear = linear;
    }

    pub fn quadratic_offset(&self) -> usize { SPOT_QUADRATIC_BYTE_OFFSET }
    pub fn quadratic(&self) -> f32 { self.quadratic }
    pub fn set_quadratic( &mut self, quadratic:f32 ) {
        self.quadratic = quadratic;
    }

    pub fn active_bytes(&self) -> [u8;4] { self.active.to_le_bytes() }
    pub fn active_offset(&self) -> usize { SPOT_ACTIVE_BYTE_OFFSET }
    pub fn is_active(&self) -> bool { self.active != 0 }
    pub fn set_active( &mut self, active:bool ) {
        self.active = if active { 1 } else { 0 }
    }

    pub fn as_bytes(&self) -> &[u8] {
        unsafe {
            any_as_u8_slice( self )
        }
    }

    pub fn size(&self) -> usize {
        core::mem::size_of::<Self>()
    }
}
