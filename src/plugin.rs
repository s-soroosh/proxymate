use hyper;
use hyper::Request;
use hyper::Response;
use std::rc::Rc;
use oauth_plugin::OauthPlugin;

pub trait Plugin {
    fn plugin_name(&self) -> String;
    fn on_request(&self, req: Request) -> Option<Request> {
        return Option::from(req);
    }
    fn on_response(&self, res: Response) -> Option<Response> {
        return Option::from(res);
    }
}


pub struct PluginRegistry {
    pub plugins: Vec<Rc<Plugin>>
}

impl PluginRegistry {
    pub fn new() -> PluginRegistry {
        PluginRegistry { plugins: Vec::new() }
    }

    pub fn register_plugin(&mut self, plugin: Rc<Plugin>) {
        self.plugins.push(plugin);
    }
}

impl Plugin for PluginRegistry {
    fn plugin_name(&self) -> String {
        return self.plugins.iter().map(|p| p.plugin_name()).fold(String::new(), |a, p| a + p.as_str()).to_string();
    }

    fn on_request(&self, req: Request) -> Option<Request> {
        let mut last_req = Option::from(req);
        for p in self.plugins.iter() {
            let o = p.on_request(last_req.unwrap());
            if o.is_none() {
                return None;
            } else {
                last_req = o;
            }
        }
        return last_req;
    }

    fn on_response(&self, res: Response) -> Option<Response> {
        unimplemented!()
    }
}

