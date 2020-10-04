use actix_web::{*, dev::*};
use futures::{Future, future::{ok, Ready}, task::{Context, Poll}};
use tracing::Span;
use std::pin::Pin;
use tracing_opentelemetry::OpenTelemetrySpanExt;
use opentelemetry::api::HttpTextFormat;


pub struct OpenTelemetryB3;

impl <S, B> Transform<S> for OpenTelemetryB3
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = OpenTelemetryB3Middleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(OpenTelemetryB3Middleware {
            service,
            propagator: opentelemetry::api::B3Propagator::new(),
        })
    }
}

pub struct OpenTelemetryB3Middleware<S> {
    service: S,
    propagator: opentelemetry::api::B3Propagator,
}

impl<S, B> Service for OpenTelemetryB3Middleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let headers = req.headers();
        let carrier = HeaderCarrier { headers };
        
        let parent_context = self.propagator
            .extract(&carrier);

        Span::current().set_parent(&parent_context);

        Box::pin(self.service.call(req))
    }
}

struct HeaderCarrier<'a> {
    pub headers: &'a actix_web::http::HeaderMap,
}

impl<'a> opentelemetry::api::Extractor for HeaderCarrier<'a> {
    fn get(&self, key: &str) -> Option<&str> {
        self.headers.get(key).and_then(|h| h.to_str().ok())
    }
}