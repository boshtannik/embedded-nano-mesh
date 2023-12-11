use super::ms;

pub trait PlatformTime {
    fn millis(&self) -> ms;
}
