use hyper::Request;
use hyper::Response;
use std::rc::Rc;

pub trait Plugin {
    fn plugin_name(&self) -> String;
    fn on_request(&self, req: Request) -> Result<Request, Response> {
        return Ok(req);
    }
    fn on_response(&self, _res: Response) -> Result<(), ()> {
        return Ok(());
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
        return self.plugins.iter()
            .map(|p| p.plugin_name())
            .fold(String::new(), |a, p| a + p.as_str())
            .to_string();
    }

    fn on_request(&self, req: Request) -> Result<Request, Response> {
        let mut reqq = req;
        for p in self.plugins.iter() {
            let o = p.on_request(reqq);
            if o.is_err() {
                return Err(o.err().unwrap());
            }
            reqq = o.ok().unwrap()
        }
        return Ok(reqq);
    }

    fn on_response(&self, _res: Response) -> Result<(), ()> {
        unimplemented!()
    }
}

