use crate::core::domain::{TypingSession, Stats};
use crate::core::application::ports::Clock;

pub struct TickUseCase<C> { clock: C }

impl<C: Clock> TickUseCase<C> {
    pub fn new(clock: C) -> Self { Self { clock } }

    pub fn execute(&self, session: &TypingSession) -> Stats {
        session.stats(self.clock.now())
    }
}
