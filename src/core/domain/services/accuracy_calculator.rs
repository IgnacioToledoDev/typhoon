pub fn accuracy(correct: u32, total: u32) -> f64 {
    if total == 0 {
        return 1.0;
    }
    let a = correct as f64 / total as f64;
    a.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_input_returns_one() {
        assert_eq!(accuracy(0, 0), 1.0);
    }

    #[test]
    fn all_correct_returns_one() {
        assert_eq!(accuracy(50, 50), 1.0);
    }

    #[test]
    fn half_correct_returns_half() {
        assert!((accuracy(50, 100) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn correct_capped_at_total() {
        assert_eq!(accuracy(120, 100), 1.0);
    }
}
