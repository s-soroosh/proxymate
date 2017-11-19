use plugin::Plugin;
use hyper::Request;
use hyper::Response;
use std::collections::HashMap;
use hyper::header::Authorization;
use hyper::{Client, StatusCode, Body};
use hyper;
use std::str::FromStr;

//#[derive(Deserialize, Debug, Clone)]
//pub struct Config {
//    pub general: GeneralConfig,
//    pub paths: HashMap<String, String>,
//}
//
//#[derive(Deserialize, Debug, Clone)]
//pub struct GeneralConfig {
//    pub listen_addr: String,
//    pub tls_key: Option<String>,
//    pub tls_password: Option<String>,
//}

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
        req.headers().clone().get::<Authorization<String>>().map(|value| {
            println!("you made it {}", value);
            Ok(req)
        }).unwrap_or(Err(Response::new().with_status(StatusCode::Unauthorized)))
    }
}

