use hyper::client::Client;
use hyper::client::HttpConnector;
use hyper::Body;
use std::io::{self, Write};
use futures::{Future, Stream};
use tokio_core::reactor::Core;
use std::thread::sleep;
use std::time;
use tlsclient;
use tokio_service::Service;
use hyper;

use native_tls::{Pkcs12, TlsAcceptor, TlsConnector};

struct ZalandoTokenInfo {}


fn get_tokeninfo(token: String, core: &mut Core, client: &Client<tlsclient::HttpsConnector, Body>) -> Result<ZalandoTokenInfo, ()> {
    let uri_str = format!("https://info.services.auth.zalando.com/oauth2/tokeninfo?access_token={}", token);
    let uri = uri_str.parse().unwrap();
    let work = client.get(uri).and_then(|res| {
        print!("status is {}", res.status());
        res.body().for_each(|chunk| {
            print!("here");
            io::stdout()
                .write_all(&chunk)
                .map(|_| ())
                .map_err(From::from)
        })
    }).map_err(|e| {
        println!("error is {}", e);
    });
    core.run(work);
    Err(())
}


#[test]
fn small_test() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let client = Client::new(&handle);
    let tls_connector = TlsConnector::builder().unwrap().build().unwrap();
    let mut http_connector = hyper::client::HttpConnector::new(4, &handle);
    http_connector.enforce_http(false);
    let https_connector = tlsclient::HttpsConnector::new(http_connector, tls_connector);

    let tls_client = hyper::Client::configure().connector(https_connector).build(&handle);

    get_tokeninfo(String::from("eyJraWQiOiJwbGF0Zm9ybS1pYW0tdmNlaHloajYiLCJhbGciOiJFUzI1NiJ9.eyJzdWIiOiI1N2FiOTI5OS0zZTdiLTRkZjctYmU0NS0yZTU5MTYxYTYyMjQiLCJodHRwczovL2lkZW50aXR5LnphbGFuZG8uY29tL3JlYWxtIjoidXNlcnMiLCJodHRwczovL2lkZW50aXR5LnphbGFuZG8uY29tL3Rva2VuIjoiQmVhcmVyIiwiaHR0cHM6Ly9pZGVudGl0eS56YWxhbmRvLmNvbS9tYW5hZ2VkLWlkIjoic3NhcmFiYWRhbmkiLCJhenAiOiJ6dG9rZW4iLCJodHRwczovL2lkZW50aXR5LnphbGFuZG8uY29tL2JwIjoiODEwZDFkMDAtNDMxMi00M2U1LWJkMzEtZDgzNzNmZGQyNGM3IiwiYXV0aF90aW1lIjoxNTExMzU2NDc5LCJpc3MiOiJodHRwczovL2lkZW50aXR5LnphbGFuZG8uY29tIiwiZXhwIjoxNTEzNjkzMzg2LCJpYXQiOjE1MTM2ODk3NzZ9.Ulc0YCTGLsLuBjtaxuce9-AbxtsXQqMQO4wg3WIGbrFBnud2Y4RmGg-3nTyrzA16CP-IRAcs5ZRnKGlhZWzmew"),
                  &mut core, &tls_client);
    sleep(time::Duration::from_millis(1000));
}