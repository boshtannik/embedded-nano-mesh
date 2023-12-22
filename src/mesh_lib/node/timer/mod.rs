use platform_millis::ms;

pub struct Timer {
    listen_period: ms,
    last_speak_time: ms,
}

/// Structure which keeps track of listening / speaking timings.
impl Timer {
    pub fn new(listen_period: ms) -> Self {
        Self {
            listen_period,
            last_speak_time: 0,
        }
    }

    /// Tells if the time since last speak is enough to speak
    /// into the ether again.
    pub fn is_time_to_speak(&self, current_time: ms) -> bool {
        current_time > { self.last_speak_time + self.listen_period }
    }

    /// Records current time as last speak time.
    pub fn record_speak_time(&mut self, current_time: ms) {
        self.last_speak_time = current_time;
    }
}
