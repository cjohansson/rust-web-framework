//! # TCP HTTP Error response
//! Used for responding a general error.

use application_layer::http::request;
use application_layer::http::response;

use std::collections::HashMap;
use std::net::SocketAddr;
use Application;

use response::tcp::http::ResponderInterface;

#[derive(Clone)]
pub struct Responder {}

impl Responder {
    pub fn new() -> Responder {
        Responder {}
    }
}

impl ResponderInterface for Responder {
    fn matches(
        &mut self,
        _request_message: &request::Message,
        _application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u64,
    ) -> bool {
        true
    }

    fn respond(
        &self,
        request_message: &request::Message,
        _application: &Application,
        _socket: &SocketAddr,
        _overflow_bytes: &u64,
    ) -> Result<response::Message, String> {
        let status_code = "500 Internal Server Error";
        let protocol = request::Message::get_protocol_text(&request_message.request_line.protocol);
        let headers: HashMap<String, String> = HashMap::new();
        let response_body = Vec::new();

        // Build HTTP response
        return Ok(response::Message::new(
            protocol.to_string(),
            status_code.to_string(),
            headers,
            response_body,
        ));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::HashMap;
    use std::net::{IpAddr, Ipv4Addr};

    use application_layer::http::response;

    use Config;

    #[test]
    fn test_matches() {
        let config = Config {
            feedback_error_file: Option::None,
            feedback_info_file: Option::None,
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: Config::get_canonical_root(&"./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
            tcp_limit: 1024,
        };
        let application = Application::new(config);
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
        let mut responder = Responder::new();
        assert!(
            responder.matches(
                &request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.0")
                    .expect("Expecting index2.htm response"),
                &application,
                &socket,
                &0
            )
        );
        assert!(
            responder.matches(
                &request::Message::from_tcp_stream(b"GET /index3.htm HTTP/1.0")
                    .expect("Expecting index3.htm response"),
                &application,
                &socket,
                &0
            )
        );
        assert!(
            responder.matches(
                &request::Message::from_tcp_stream(b"GET /index.htm HTTP/1.1")
                    .expect("Expecting index.htm response"),
                &application,
                &socket,
                &0
            )
        );
    }

    #[test]
    fn test_respond() {
        let config = Config {
            feedback_error_file: Option::None,
            feedback_info_file: Option::None,
            filesystem_directory_index: "index.htm".to_string(),
            file_not_found_file: "404.htm".to_string(),
            filesystem_root: Config::get_canonical_root(&"./html/".to_string()).unwrap(),
            server_host: "localhost".to_string(),
            server_limit: 4,
            server_port: 4040,
            tcp_limit: 1024,
        };
        let application = Application::new(config);
        let mut responder = Responder::new();
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);

        // Build response body
        let response_body = String::new();
        let request =
            request::Message::from_tcp_stream(b"GET /index2.htm HTTP/1.1\r\n\r\n").unwrap();
        let matches = responder.matches(&request, &application, &socket, &0);
        assert!(matches);

        let headers: HashMap<String, String> = HashMap::new();

        let expected_response = response::Message::new(
            "HTTP/1.1".to_string(),
            "500 Internal Server Error".to_string(),
            headers,
            response_body.into_bytes(),
        ).to_bytes();

        let given_response = responder
            .respond(&request, &application, &socket, &0)
            .unwrap()
            .to_bytes();
        assert_eq!(expected_response, given_response);
    }
}
