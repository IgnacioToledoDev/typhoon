pub trait Rng {
    fn next_u64(&mut self) -> u64;
}
