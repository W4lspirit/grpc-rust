/*
 *
 * Copyright 2025 gRPC authors.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to
 * deal in the Software without restriction, including without limitation the
 * rights to use, copy, modify, merge, publish, distribute, sublicense, and/or
 * sell copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 *
 */

//! `tokio` based runtime implementation.

use crate::runtime::Runtime;
use std::future::Future;
use std::time::{Duration, Instant};

/// Tokio-based runtime implementation.
#[derive(Clone, Debug, Default)]
pub struct TokioRuntime;

impl Runtime for TokioRuntime {
    fn spawn<F>(&self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        tokio::spawn(future);
    }

    async fn sleep(&self, duration: Duration) {
        tokio::time::sleep(duration).await;
    }

    fn now(&self) -> Instant {
        tokio::time::Instant::now().into_std()
    }

    async fn sleep_until(&self, deadline: Instant) {
        tokio::time::sleep_until(deadline.into()).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn delayed_poll_does_not_extend_deadline() {
        let runtime = TokioRuntime;
        for scheduling_delay in [3, 12] {
            let deadline = runtime.now() + Duration::from_secs(10);
            let sleep = runtime.sleep_until(deadline);
            tokio::time::advance(Duration::from_secs(scheduling_delay)).await;
            let resumed_at = runtime.now();
            sleep.await;
            assert_eq!(runtime.now(), deadline.max(resumed_at));
        }
    }
}
