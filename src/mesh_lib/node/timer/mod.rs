use crate::mesh_lib::millis::ms;

pub struct Timer {
    listen_period: ms,
    last_speak_time: ms,
    millis_fn_ptr: fn() -> ms,
}

impl Timer {
    pub fn new(listen_period: ms, millis_fn: fn() -> ms) -> Self {
        Self {
            listen_period,
            last_speak_time: 0,
            millis_fn_ptr: millis_fn,
        }
    }

    pub fn is_time_to_speak(&self) -> bool {
        let current_time = { self.millis_fn_ptr }();
        current_time > { self.last_speak_time + self.listen_period }
    }

    pub fn record_speak_time(&mut self) {
        let current_time = { self.millis_fn_ptr }();
        self.last_speak_time = current_time;
    }
}
