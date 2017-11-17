use hyper;
use hyper::Request;
use hyper::Response;


pub trait Plugin {
    fn plugin_name(&self) -> String;

    fn on_request(&self, req: Request) -> Option<Request> {
        return Option::from(req);
    }
    fn on_response(&self, res: Response) -> Option<Response> {
        return Option::from(res);
    }
}


