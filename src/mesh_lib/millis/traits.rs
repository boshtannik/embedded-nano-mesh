use super::ms;

pub trait PlatformTime {
    type Timer;
    fn init(timer: Self::Timer);
    fn millis() -> ms;
}
