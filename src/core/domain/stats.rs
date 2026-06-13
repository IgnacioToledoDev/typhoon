use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Stats {
    pub elapsed: Duration,
    pub remaining: Duration,
    pub typed_chars: u32,
    pub correct_chars: u32,
    pub errors: u32,
    pub gross_wpm: f64,
    pub net_wpm: f64,
    pub accuracy: f64,
}
