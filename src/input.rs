use fmath::types::*;

pub struct Input {
    
    pub forward: bool, pub back:  bool,
    pub left:    bool, pub right: bool,
    pub up:      bool, pub down:  bool,

    pub speed_up: bool,

    mouse_pos:   Vector2,
    mouse_delta: Vector2,

    quit: bool,
}

impl Input {
    pub fn new() -> Input {
        Input {
            forward: false, back:  false,
            left:    false, right: false,
            up:      false, down:  false,

            speed_up: false,

            mouse_pos:   Vector2::new_zero(),
            mouse_delta: Vector2::new_zero(),

            quit: false,
        }
    }

    pub fn direction( &self ) -> Vector3 {
        let direction = | negative:bool, positive:bool | -> f32 {
            if positive == negative { return 0.0 }
            else if positive { return  1.0 }
            else { return -1.0 }
        };
        Vector3::new(
            direction( self.left, self.right ),
            direction( self.down, self.up ),
            direction( self.back, self.forward ),
        )
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