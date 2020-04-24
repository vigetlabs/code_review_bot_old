use actix_service::{Service, Transform};
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, http, Error, HttpResponse};
use futures::future::{ok, Either, Future, Ready};
use std::task::{Context, Poll};
use std::pin::Pin;

use crate::utils::app_config::AppConfig;

pub struct SetupRedirect;

type PinBox<T> = Pin<Box<T>>;

impl<S, B> Transform<S> for SetupRedirect
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = SetupRedirectMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(SetupRedirectMiddleware { service })
    }
}

pub struct SetupRedirectMiddleware<S> {
    service: S,
}

impl<S, B> Service for SetupRedirectMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = PinBox<dyn Future<Output = Result<Self::Response, Self::Error>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        let config = req
            .app_data::<AppConfig>()
            .expect("AppConfig must be setup");

        let app_data = {
            let data_builder = config.builder.lock().expect("Builder can't be accessed");
            let mut app_data = config.data.lock().expect("Data can't be accessed");

            if data_builder.is_complete() && app_data.is_none() {
                *app_data = data_builder.clone().build();
            }
            app_data.clone()
        };

        let fut = if app_data.is_some() || req.path() == "/setup" {
            Either::Left(self.service.call(req))
        } else {
            Either::Right(ok(req.into_response(
                HttpResponse::Found()
                    .header(http::header::LOCATION, "/setup")
                    .finish()
                    .into_body(),
            )))
        };

        Box::pin(fut)
    }
}
