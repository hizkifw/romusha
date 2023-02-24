use tokio::time::{Duration, Instant};

pub struct Backoff {
    pub min: Duration,
    pub max: Duration,
    pub reset_after: Duration,
    pub current: Duration,
    pub last_failure: Instant,
}

impl Backoff {
    pub fn new() -> Self {
        return Backoff {
            min: Duration::from_secs(1),
            max: Duration::from_secs(60),
            reset_after: Duration::from_secs(300),
            current: Duration::from_secs(1),
            last_failure: Instant::now(),
        };
    }

    pub async fn fail(&mut self) {
        tokio::time::sleep(self.current).await;
        self.current *= 2;
        if self.current > self.max {
            self.current = self.max.clone();
        }
        self.last_failure = Instant::now();
    }

    pub fn reset(&mut self) {
        if Instant::now() - self.last_failure > self.reset_after {
            self.force_reset();
        }
    }

    pub fn force_reset(&mut self) {
        self.current = self.min.clone();
    }
}
