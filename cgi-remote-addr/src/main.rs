extern crate cgi;

cgi::cgi_main! { |request: cgi::Request| -> cgi::Response {
    let world = "world";
    cgi::text_response(200, format!("Hello {}", world))
} }
