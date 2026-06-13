// Stateless domain-adjacent calculators.

pub mod wpm_calculator;
pub mod accuracy_calculator;

pub use wpm_calculator::{gross_wpm, net_wpm};
pub use accuracy_calculator::accuracy;
