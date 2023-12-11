use crate::mesh_lib::millis::ms;

pub struct Timer {
    listen_period: ms,
    last_speak_time: ms,
}

impl Timer {
    pub fn new(listen_period: ms) -> Self {
        Self {
            listen_period,
            last_speak_time: 0,
        }
    }

    pub fn is_time_to_speak(&self, current_time: ms) -> bool {
        let current_time = current_time;
        current_time > { self.last_speak_time + self.listen_period }
    }

    pub fn record_speak_time(&mut self, current_time: ms) {
        let current_time = current_time;
        self.last_speak_time = current_time;
    }
}
