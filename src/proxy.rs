use futures;
use futures::{Future, BoxFuture};
use futures::future::FutureResult;

use hyper;
use hyper::{Client, StatusCode, Body};
use hyper::client::HttpConnector;
use hyper::server::{Service, Request, Response};
use hyper::Uri;
use std::str::FromStr;

use chrono::prelude::*;

use regex;

use tlsclient::HttpsConnector;
use errors;
use plugin::Plugin;
use oauth_plugin::OauthPlugin;
use std::sync::Arc;

#[derive(Clone)]
pub struct Routes {
    pub routes: Vec<(regex::Regex, String)>,
    pub regexes: regex::RegexSet,
}

pub struct Proxy {
    pub routes: Routes,
    pub client: Client<HttpConnector, Body>,
    pub tls_client: Client<HttpsConnector, Body>,
    pub plugin: Arc<Plugin>
}

impl Service for Proxy{
    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Response, Error=Self::Error>>;


    fn call(&self, req: Request) -> Self::Future {
        let uri = req.uri();
        //        println!("uri is {}", uri);
        let matches = self.routes.regexes.matches(uri.path());
        //        println!("routes are {:?}", self.routes.regexes);

        let fut = {
            if !matches.matched_any() {
                futures::future::ok(Response::new().with_status(StatusCode::NotFound)).boxed()
            } else {
                // Find the most specific match (unwrap called here because of the above check)
                let index = matches.iter().next().unwrap();
                let (ref regex, ref other_site) = self.routes.routes[index];
                //                let url = hyper::Uri::parse(other_site).expect("configuration problem, other site not valid URL");
                //                let url = other_site.parse::<hyper::Uri>().expect("configuration problem, other site not valid URL");
                if let Some(caps) = regex.captures(uri.path()) {
                    let site_url = match caps.name("site_url") {
                        Some(m) => m.as_str(),
                        None => {
                            error!("no site_url present");
                            return futures::future::ok(
                                Response::new().with_status(StatusCode::InternalServerError)).boxed();
                        }
                    };
                    let url = hyper::Uri::from_str(format!("{}{}", other_site, site_url).as_str()).expect("generated uri is not valid!!!");
                    println!("forward request to {}", url);
                    let secure = url.scheme().unwrap_or("") == "https";
                    let mut proxied_request = hyper::client::Request::new(req.method().clone(), url);
                    *proxied_request.headers_mut() = req.headers().clone();

                    //call plugins here
                    let o = self.plugin.on_request(&mut proxied_request);
                    if o.is_err() {
                        return futures::future::ok(Response::new().with_status(StatusCode::BadRequest)).boxed();
                    }

                    let req = if secure {
                        self.tls_client.request(proxied_request)
                    } else {
                        self.client.request(proxied_request)
                    };
                    Box::new(req.then(|res| {
                        println!("got response back!");
                        if let Ok(res) = res {
                            futures::future::ok(
                                Response::new()
                                    .with_status(res.status().clone())
                                    .with_headers(res.headers().clone())
                                    .with_body(res.body()))
                        } else {
                            futures::future::ok(
                                Response::new()
                                    .with_status(StatusCode::ServiceUnavailable))
                        }
                    })) as Self::Future
                } else {
                    futures::future::ok(Response::new().with_status(StatusCode::BadGateway)).boxed()
                }
            }
        };
        fut
    }
}