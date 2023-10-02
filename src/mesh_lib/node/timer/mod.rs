use crate::mesh_lib::millis::{millis, ms};

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

    pub fn is_time_to_speak(&self) -> bool {
        millis() > self.last_speak_time + self.listen_period
    }

    pub fn record_speak_time(&mut self) {
        self.last_speak_time = millis();
    }
}
