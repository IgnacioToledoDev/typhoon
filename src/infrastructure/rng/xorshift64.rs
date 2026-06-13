use std::time::{SystemTime, UNIX_EPOCH};
use crate::core::application::ports::Rng;

pub struct Xorshift64 {
    state: u64,
}

const DEFAULT_STATE: u64 = 0x9E37_79B9_7F4A_7C15;

impl Xorshift64 {
    pub fn from_seed(seed: u64) -> Self {
        let state = if seed == 0 { DEFAULT_STATE } else { seed };
        Self { state }
    }

    pub fn from_entropy() -> Self {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u64)
            .unwrap_or(DEFAULT_STATE);
        let pid = std::process::id() as u64;
        let mixed = nanos ^ pid.rotate_left(17);
        Self::from_seed(mixed)
    }
}

impl Rng for Xorshift64 {
    fn next_u64(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_seed_zero_substitutes_default_state() {
        let mut a = Xorshift64::from_seed(0);
        assert_ne!(a.next_u64(), 0);
    }

    #[test]
    fn deterministic_for_same_seed() {
        let mut a = Xorshift64::from_seed(42);
        let mut b = Xorshift64::from_seed(42);
        for _ in 0..1000 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn distribution_into_buckets_is_roughly_uniform() {
        let mut rng = Xorshift64::from_seed(0xC0FFEE);
        let mut buckets = [0u32; 10];
        for _ in 0..10_000 {
            let idx = (rng.next_u64() % 10) as usize;
            buckets[idx] += 1;
        }
        for (i, &count) in buckets.iter().enumerate() {
            assert!(
                count > 800 && count < 1200,
                "bucket {i} had {count} samples, expected ~1000"
            );
        }
    }
}
