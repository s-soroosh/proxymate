use hyper::client::Client;
use hyper::client::HttpConnector;
use hyper::Body;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;

struct ZalandoTokenInfo {}


fn get_tokeninfo(token: String, core: & mut Core, client: &Client<HttpConnector, Body>) -> Result<ZalandoTokenInfo, ()> {
    let uri = "http://roozame.com/api/news".parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        res.body().for_each(|chunk| {
            io::stdout()
                .write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
        })
    });
    core.run(work);
    Err(())
}


#[test]
fn small_test() {
    let mut core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    get_tokeninfo(String::from("h"), & mut core, &client);
}