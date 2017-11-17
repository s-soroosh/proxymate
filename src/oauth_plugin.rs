use plugin::Plugin;
use hyper::Request;
use hyper::Response;

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
    fn on_request(&self, req: Request) -> Option<Request> {
        return None;
    }
}

