use std::iter::FusedIterator;
use std::time::Duration;

struct RetryBackoff {
    current: Duration,
    max_delay: Duration,
    remaining: usize,
}

impl RetryBackoff {
    fn new(initial: Duration, max_delay: Duration, attempts: usize) -> Self {
        Self {
            current: initial,
            max_delay,
            remaining: attempts,
        }
    }
}

impl Iterator for RetryBackoff {
    type Item = Duration;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        self.remaining -= 1;
        let delay = self.current.min(self.max_delay);
        self.current = self.current.saturating_mul(2).min(self.max_delay);
        Some(delay)
    }
}

// Гарантируем, что после исчерпания попыток итератор всегда будет возвращать None
impl FusedIterator for RetryBackoff {}

fn main() {
    let attempts = 5;
    let schedule: Vec<Duration> =
        RetryBackoff::new(Duration::from_millis(100), Duration::from_millis(500), attempts)
            .collect();
    println!("План задержек retry: {:?}", schedule);
}
