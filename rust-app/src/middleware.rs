use actix_web::{
    Error,
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
};
use futures::future::{LocalBoxFuture, Ready, ok};
use lazy_static::lazy_static;
use prometheus::{HistogramVec, register_histogram_vec};
use std::rc::Rc;
use std::task::{Context, Poll};
use std::time::Instant;

lazy_static! {
    static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "http_requests_duration_seconds",
        "Histogram of HTTP request durations in seconds",
        &["error", "exception", "method", "outcome", "status", "uri"]
    )
    .unwrap();
}

pub struct Metrics;

impl<S, B> Transform<S, ServiceRequest> for Metrics
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = MetricsMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(MetricsMiddleware {
            service: Rc::new(service),
        })
    }
}

pub struct MetricsMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for MetricsMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let method = req.method().to_string();
        let path = req.path().to_string();
        let svc = Rc::clone(&self.service);
        let start = Instant::now();

        Box::pin(async move {
            let mut res = svc.call(req).await;
            let elapsed = start.elapsed().as_secs_f64();
            let service_response = res.as_mut().ok().unwrap();

            HTTP_REQ_HISTOGRAM
                .with_label_values(&[
                    "none",
                    "none",
                    method.as_str(),
                    "SUCCESS",
                    service_response.status().as_str(),
                    path.as_str(),
                ])
                .observe(elapsed);

            res
        })
    }
}
