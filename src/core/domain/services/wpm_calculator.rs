use std::time::Duration;

const MIN_ELAPSED_SECS: f64 = 1e-3;
const CHARS_PER_WORD: f64 = 5.0;

pub fn gross_wpm(chars_typed: u32, elapsed: Duration) -> f64 {
    let secs = elapsed.as_secs_f64();
    if secs < MIN_ELAPSED_SECS {
        return 0.0;
    }
    let minutes = secs / 60.0;
    (chars_typed as f64 / CHARS_PER_WORD) / minutes
}

pub fn net_wpm(chars_typed: u32, errors: u32, elapsed: Duration) -> f64 {
    let secs = elapsed.as_secs_f64();
    if secs < MIN_ELAPSED_SECS {
        return 0.0;
    }
    let minutes = secs / 60.0;
    let gross = (chars_typed as f64 / CHARS_PER_WORD) / minutes;
    let errors_per_min = errors as f64 / minutes;
    (gross - errors_per_min).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gross_wpm_60_seconds_300_chars_returns_60() {
        let wpm = gross_wpm(300, Duration::from_secs(60));
        assert!((wpm - 60.0).abs() < 1e-6, "got {wpm}");
    }

    #[test]
    fn gross_wpm_30_seconds_300_chars_returns_120() {
        let wpm = gross_wpm(300, Duration::from_secs(30));
        assert!((wpm - 120.0).abs() < 1e-6, "got {wpm}");
    }

    #[test]
    fn gross_wpm_zero_elapsed_returns_zero() {
        let wpm = gross_wpm(50, Duration::from_micros(0));
        assert_eq!(wpm, 0.0);
    }

    #[test]
    fn net_wpm_subtracts_errors_per_minute() {
        let wpm = net_wpm(300, 10, Duration::from_secs(60));
        assert!((wpm - 50.0).abs() < 1e-6, "got {wpm}");
    }

    #[test]
    fn net_wpm_never_negative() {
        let wpm = net_wpm(50, 100, Duration::from_secs(60));
        assert_eq!(wpm, 0.0);
    }

    #[test]
    fn net_wpm_zero_elapsed_returns_zero() {
        assert_eq!(net_wpm(300, 0, Duration::from_micros(0)), 0.0);
    }
}
