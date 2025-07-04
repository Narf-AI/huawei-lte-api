//! Retry logic with exponential backoff.
//! 
//! This module implements a configurable retry strategy with exponential backoff
//! and optional jitter. It's used throughout the library to handle transient
//! network failures and temporary API errors.
//! 
//! # Example
//! 
//! ```
//! use huawei_dongle_api::retry::RetryStrategy;
//! use std::time::Duration;
//! 
//! let strategy = RetryStrategy {
//!     max_attempts: 5,
//!     initial_delay: Duration::from_millis(100),
//!     max_delay: Duration::from_secs(10),
//!     backoff_multiplier: 2.0,
//!     jitter: true,
//! };
//! ```

use crate::error::{Error, Result};
use std::time::Duration;
use tokio::time::sleep;
use tracing::debug;

/// Retry strategy configuration.
/// 
/// Controls how failed requests are retried, including the number of attempts,
/// delays between attempts, and backoff behavior.
#[derive(Debug, Clone)]
pub struct RetryStrategy {
    /// Maximum number of retry attempts
    pub max_attempts: usize,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f64,
    /// Whether to add random jitter to delays to prevent thundering herd
    pub jitter: bool,
}

impl Default for RetryStrategy {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(30),
            backoff_multiplier: 2.0,
            jitter: true,
        }
    }
}

impl RetryStrategy {
    /// Calculate the delay for a given attempt
    fn calculate_delay(&self, attempt: usize) -> Duration {
        let base_delay = self.initial_delay.as_millis() as f64;
        let multiplier = self.backoff_multiplier.powi(attempt as i32);
        let delay_ms = (base_delay * multiplier) as u64;

        let delay = Duration::from_millis(delay_ms).min(self.max_delay);

        if self.jitter {
            let jitter_factor = 0.75 + (fastrand::f64() * 0.5);
            let jittered_ms = (delay.as_millis() as f64 * jitter_factor) as u64;
            Duration::from_millis(jittered_ms)
        } else {
            delay
        }
    }

    /// Execute a function with retry logic
    pub async fn execute<F, Fut, T>(&self, operation: F) -> Result<T>
    where
        F: Fn() -> Fut,
        Fut: std::future::Future<Output = Result<T>>,
    {
        let mut last_error = None;

        for attempt in 0..self.max_attempts {
            match operation().await {
                Ok(result) => {
                    if attempt > 0 {
                        debug!("Operation succeeded after {} retries", attempt);
                    }
                    return Ok(result);
                }
                Err(error) => {
                    if !error.is_retryable() {
                        debug!("Error is not retryable, failing immediately: {}", error);
                        return Err(error);
                    }

                    debug!("Attempt {} failed: {}", attempt + 1, error);
                    last_error = Some(error);

                    if attempt < self.max_attempts - 1 {
                        let delay = self.calculate_delay(attempt);
                        debug!("Retrying in {:?}", delay);
                        sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error.unwrap_or_else(|| Error::generic("All retry attempts failed")))
    }
}

/// Helper function to use with .and_then() on Results
pub async fn with_retry<F, Fut, T>(strategy: &RetryStrategy, operation: F) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    strategy.execute(operation).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_retry_success_on_first_attempt() {
        let strategy = RetryStrategy::default();
        let attempt_count = Arc::new(AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = strategy
            .execute(|| async {
                attempt_count_clone.fetch_add(1, Ordering::SeqCst);
                Ok::<i32, Error>(42)
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn test_retry_success_after_failures() {
        let strategy = RetryStrategy {
            max_attempts: 3,
            initial_delay: Duration::from_millis(10),
            jitter: false,
            ..Default::default()
        };

        let attempt_count = Arc::new(AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = strategy
            .execute(|| async {
                let count = attempt_count_clone.fetch_add(1, Ordering::SeqCst);
                if count < 2 {
                    Err(Error::session("Temporary failure"))
                } else {
                    Ok::<i32, Error>(42)
                }
            })
            .await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(attempt_count.load(Ordering::SeqCst), 3);
    }

    #[tokio::test]
    async fn test_retry_non_retryable_error() {
        let strategy = RetryStrategy::default();
        let attempt_count = Arc::new(AtomicUsize::new(0));
        let attempt_count_clone = attempt_count.clone();

        let result = strategy
            .execute(|| async {
                attempt_count_clone.fetch_add(1, Ordering::SeqCst);
                Err::<i32, Error>(Error::authentication("Invalid credentials"))
            })
            .await;

        assert!(result.is_err());
        assert_eq!(attempt_count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_delay_calculation() {
        let strategy = RetryStrategy {
            initial_delay: Duration::from_millis(100),
            backoff_multiplier: 2.0,
            max_delay: Duration::from_secs(10),
            jitter: false,
            ..Default::default()
        };

        assert_eq!(strategy.calculate_delay(0), Duration::from_millis(100));
        assert_eq!(strategy.calculate_delay(1), Duration::from_millis(200));
        assert_eq!(strategy.calculate_delay(2), Duration::from_millis(400));
    }
}
