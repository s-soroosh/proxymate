use std::io;
use std::rc::Rc;
use hyper::Uri;
use hyper::client::{Connect, HttpConnector};
use native_tls::TlsConnector;
use tokio_tls::{TlsStream, TlsConnectorExt};
use tokio_service::Service;
use tokio_core::net::TcpStream;
use futures::future::{err, Future};


#[derive(Clone)]
pub struct HttpsConnector {
    http: HttpConnector,
    tls_connector: Rc<TlsConnector>,
}

impl HttpsConnector {

    pub fn new(http: HttpConnector, tls_connector: TlsConnector) -> HttpsConnector {
        HttpsConnector {
            http: http,
            tls_connector: Rc::new(tls_connector),
        }
    }
}

impl Service for HttpsConnector {
    type Request = Uri;
    type Response = TlsStream<TcpStream>;
    type Error = io::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, url: Uri) -> Self::Future {
//        let http_connect = self.http.connect(url.clone());
//        let tls_connector = self.tls_connector.clone();
//        Box::new(
//            http_connect.and_then(move |s| tls_connector.connect_async(url.host().unwrap(), s).map_err(|e| io::Error::new(io::ErrorKind::Other, e)))
//        )
        if url.scheme() != Some("https") {
            return err(io::Error::new(io::ErrorKind::Other,
                                      "only works with https")).boxed()
        }

        // Look up the host that we're connecting to as we're going to validate
        // this as part of the TLS handshake.
        let host = match url.host() {
            Some(s) => s.to_string(),
            None =>  {
                return err(io::Error::new(io::ErrorKind::Other,
                                          "missing host")).boxed()
            }
        };

        // Delegate to the standard `HttpConnector` type to create a connected
        // TCP socket. Once we've got that socket initiate the TLS handshake
        // with the host name that's provided in the URI we extracted above.
        let tls_cx = self.tls_connector.clone();
        Box::new(self.http.call(url).and_then(move |tcp| {
            tls_cx.connect_async(&host, tcp)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
        }))
    }

}