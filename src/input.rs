use fmath::types::*;

pub struct Input {
    
    pub front:bool, pub back: bool,
    pub left: bool, pub right:bool,
    pub up:   bool, pub down: bool,

    mouse_pos:Vector2,
    mouse_delta:Vector2,
    ortho_enabled:bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            front:false, back: false,
            left: false, right:false,
            up:   false, down: false,

            mouse_pos:Vector2::new_zero(),
            mouse_delta:Vector2::new_zero(),
            ortho_enabled:false,
        }
    }

    pub fn toggle_ortho( &mut self ) {
        self.ortho_enabled = !self.ortho_enabled;
    }

    pub fn set_mouse( &mut self, pos:Vector2 ) {
        self.mouse_pos = pos;
    }

    pub fn set_mouse_delta( &mut self, delta:Vector2 ) {
        self.mouse_delta = delta;
    }

    pub fn mouse( &self ) -> &Vector2 {
        &self.mouse_pos
    }

    pub fn mouse_delta( &self ) -> &Vector2 {
        &self.mouse_delta
    }

    pub fn is_ortho_enabled( &self ) -> bool {
        self.ortho_enabled
    }
}