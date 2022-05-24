use core::fmt;

pub struct Time {
    
    elapsed_ticks: u32,
    elapsed_ms: ms,

    last_elapsed_ticks: u32,

    delta_ticks: u32,
    delta_ms:    ms,

}

impl Time {

    pub fn new() -> Self {
        Self {
            elapsed_ticks: 0,
            elapsed_ms: ms::new(),
            last_elapsed_ticks: 0,
            delta_ticks: 0,
            delta_ms: ms::new(),
        }
    }

    pub fn is_first_frame( &self ) -> bool {
        self.last_elapsed_ticks == 0
    }

    pub fn update( &mut self, ticks:u32 ) {
        self.last_elapsed_ticks = self.elapsed_ticks;
        self.elapsed_ticks      = ticks;
        self.delta_ticks        = self.elapsed_ticks - self.last_elapsed_ticks;

        self.elapsed_ms.dirty = true;
        self.delta_ms.dirty   = true;
    }

    pub fn time( &mut self ) -> f32 {
        if self.elapsed_ms.dirty {
            self.elapsed_ms.t = self.elapsed_ticks as f32 / 1000.0
        }
        self.elapsed_ms.t
    }

    pub fn delta_time( &mut self ) -> f32 {
        if self.delta_ms.dirty {
            self.delta_ms.t = self.delta_ticks as f32 / 1000.0
        }
        self.delta_ms.t
    }

}

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "elapsed: {:7.4} delta: {:7.4}", self.elapsed_ms.t, self.delta_ms.t)
    }
}

#[allow(non_camel_case_types)]
struct ms {
    t:f32,
    dirty:bool
}

impl ms {
    pub fn new() -> Self {
        Self { t: 0.0, dirty: true, }
    }
}