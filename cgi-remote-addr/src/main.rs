extern crate cgi;

cgi::cgi_main! { |request: cgi::Request| -> cgi::Response {
    let headers = request.headers();
    let mut x: String = "".to_string();
    for key in headers.keys() {
        x = x + "\n" + key.as_str();
    }
    let remote_addr = request.headers().get("x-cgi-remote-addr").unwrap();
    cgi::text_response(200, format!("Hello {}, {:?}", x, remote_addr))
} }
