use super::ms;

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

#[cfg(test)]
mod tests {
    use super::*;

    const PERIOD: ms = 100;

    #[test]
    fn does_not_speak_before_period_elapses() {
        let mut timer = Timer::new(PERIOD);
        timer.record_speak_time(1000);
        assert!(!timer.is_time_to_speak(1099));
    }

    #[test]
    fn speaks_when_period_elapses() {
        let mut timer = Timer::new(PERIOD);
        timer.record_speak_time(1000);
        assert!(timer.is_time_to_speak(1100));
    }

    #[test]
    fn does_not_speak_before_period_elapses_across_u32_wraparound() {
        let mut timer = Timer::new(PERIOD);
        // last_speak 50ms before u32 overflow
        let last_speak: ms = u32::MAX - 50;
        timer.record_speak_time(last_speak);
        // 99ms later — wraps to u32::MAX - 50 + 99 = 48
        let current = last_speak.wrapping_add(PERIOD - 1);
        assert!(
            !timer.is_time_to_speak(current),
            "fired after only {}ms; old code: ({} + {}) overflows to {}, then {} > {} is true",
            PERIOD - 1,
            last_speak,
            PERIOD,
            last_speak.wrapping_add(PERIOD),
            current,
            last_speak.wrapping_add(PERIOD),
        );
    }

    #[test]
    fn speaks_after_period_elapses_across_u32_wraparound() {
        let mut timer = Timer::new(PERIOD);
        let last_speak: ms = u32::MAX - 50;
        timer.record_speak_time(last_speak);
        // exactly 100ms later — wraps to 49
        let current = last_speak.wrapping_add(PERIOD);
        assert!(timer.is_time_to_speak(current));
    }
}
