use fmath::types::*;

pub struct Input {
    
    pub front:bool, pub back: bool,
    pub left: bool, pub right:bool,
    pub up:   bool, pub down: bool,

    mouse_pos:Vector2,
    mouse_delta:Vector2,

    quit:bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            front:false, back: false,
            left: false, right:false,
            up:   false, down: false,

            mouse_pos:Vector2::new_zero(),
            mouse_delta:Vector2::new_zero(),

            quit:false,
        }
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

    pub fn quit_game( &mut self ) {
        self.quit = true;
    }

    pub fn is_quitting( &self ) -> bool {
        self.quit
    }
}