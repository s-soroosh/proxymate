use plugin::Plugin;
use hyper::Request;
use hyper::Response;
use std::collections::HashMap;
use hyper::header::Authorization;
//use hyper::{Client, StatusCode, Body};
use hyper::{StatusCode};
//use hyper;
//use std::str::FromStr;


#[derive(Deserialize, Debug, Clone)]
pub struct OauthPluginConfig {
    pub oauth: OauthConfig,
    pub realms: HashMap<String, RealmConfig>
}

#[derive(Deserialize, Debug, Clone)]
pub struct OauthConfig {
    pub tokeninfo_url: String
}

#[derive(Deserialize, Debug, Clone)]
pub struct RealmConfig {
    pub scopes: Option<Vec<String>>,
    pub uids: Option<Vec<String>>
}

#[derive(Clone)]
pub struct OauthPlugin {}

impl OauthPlugin {
    pub fn new() -> OauthPlugin {
        OauthPlugin {}
    }
}

impl Plugin for OauthPlugin {
    fn plugin_name(&self) -> String {
        return String::from("Oauth Plugin");
    }
    fn on_request(&self, req: Request) -> Result<Request, Response> {
        req.headers().clone().get::<Authorization<String>>().map(|_| {
            Ok(req)
        }).unwrap_or(Err(Response::new().with_status(StatusCode::Unauthorized)))
    }
}

#[test]
fn oauth_plugin_respond_with_unauthorized_when_authorization_header_is_missing(){
    use std::str::FromStr;
    use hyper::*;

    let response = OauthPlugin::new().on_request(
        Request::new(Method::Get, Uri::from_str("example.com").unwrap())
    );
    assert_eq!(
    response.unwrap_err().status(),
    StatusCode::Unauthorized
    );
}