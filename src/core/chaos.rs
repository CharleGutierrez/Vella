use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use futures_util::future::BoxFuture;
use rand::Rng;
use std::{
    task::{Context, Poll},
    time::Duration,
};
use tower::{Layer, Service};
use tracing::{info, warn};

/// Real Chaos Monkey Middleware Layer for Axum
#[derive(Clone)]
pub struct ChaosMonkeyLayer {
    pub fault_probability: f64, // 0.0 to 1.0
    pub max_latency_ms: u64,
}

impl ChaosMonkeyLayer {
    pub fn new(fault_probability: f64, max_latency_ms: u64) -> Self {
        warn!("⚠️ INITIALIZING REAL CHAOS MONKEY: Fault Probability: {}%, Max Latency: {}ms", fault_probability * 100.0, max_latency_ms);
        Self {
            fault_probability,
            max_latency_ms,
        }
    }
}

impl<S> Layer<S> for ChaosMonkeyLayer {
    type Service = ChaosMonkeyMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ChaosMonkeyMiddleware {
            inner,
            fault_probability: self.fault_probability,
            max_latency_ms: self.max_latency_ms,
        }
    }
}

#[derive(Clone)]
pub struct ChaosMonkeyMiddleware<S> {
    inner: S,
    fault_probability: f64,
    max_latency_ms: u64,
}

impl<S> Service<Request<Body>> for ChaosMonkeyMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + Clone + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let mut inner = self.inner.clone();
        let fault_prob = self.fault_probability;
        let max_lat = self.max_latency_ms;

        Box::pin(async move {
            let (should_drop, delay) = {
                let mut rng = rand::thread_rng();
                let roll: f64 = rng.gen();
                if roll < fault_prob {
                    let severity: u32 = rng.gen_range(1..=10);
                    if severity <= 3 {
                        (true, 0)
                    } else {
                        (false, rng.gen_range(10..=max_lat))
                    }
                } else {
                    (false, 0)
                }
            };

            if should_drop {
                warn!("🧨 Chaos Monkey: Dropping request completely! (Returning HTTP 503 Service Unavailable)");
                return Ok((
                    StatusCode::SERVICE_UNAVAILABLE,
                    "Chaos Monkey: Network Partition Simulation",
                )
                    .into_response());
            } else if delay > 0 {
                warn!("⏳ Chaos Monkey: Injecting artificial network latency spike of {}ms", delay);
                tokio::time::sleep(Duration::from_millis(delay)).await;
            }

            inner.call(req).await
        })
    }
}
