//! Default Compute template program.

use fastly::http::{header, Method, StatusCode};
use fastly::{mime, Error, Request, Response};

macro_rules! serve_file {
    ($path:expr, $content_type:expr, $file:expr) => {
        Ok(Response::from_status(StatusCode::OK)
            .with_content_type($content_type)
            .with_body(include_str!($file)))
    };
    ($path:expr, $content_type:expr, bytes $file:expr) => {
        Ok(Response::from_status(StatusCode::OK)
            .with_content_type($content_type)
            .with_body(include_bytes!($file).as_slice()))
    };
}

/// The entry point for your application.
///
/// This function is triggered when your service receives a client request. It could be used to
/// route based on the request properties (such as method or path), send the request to a backend,
/// make completely new requests, and/or generate synthetic responses.
///
/// If `main` returns an error, a 500 error response will be delivered to the client.
#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    // Log service version
    println!(
        "FASTLY_SERVICE_VERSION: {}",
        std::env::var("FASTLY_SERVICE_VERSION").unwrap_or_else(|_| String::new())
    );

    // Filter request methods...
    match req.get_method() {
        // Block requests with unexpected methods
        &Method::POST | &Method::PUT | &Method::PATCH | &Method::DELETE => {
            return Ok(Response::from_status(StatusCode::METHOD_NOT_ALLOWED)
                .with_header(header::ALLOW, "GET, HEAD, PURGE")
                .with_body_text_plain("This method is not allowed\n"))
        }

        // Let any other requests through
        _ => (),
    };

    // Pattern match on the path...
    match req.get_path() {

        "/" => serve_file!("/index.html", mime::TEXT_HTML_UTF_8, "../../index.html"),
        "/data/data.json" => serve_file!("/data/data.json", mime::TEXT_HTML_UTF_8, "../../data/data.json"),
        "/data/intro.json" => serve_file!("/data/intro.json", mime::TEXT_HTML_UTF_8, "../../data/intro.json"),
        "/data/outro.json" => serve_file!("/data/outro.json", mime::TEXT_HTML_UTF_8, "../../data/outro.json"),

        "/space_invaders.wasm" => serve_file!("/space_invaders.wasm", mime::APPLICATION_OCTET_STREAM, bytes "../../space_invaders.wasm"),
        "/data/footer.png" => serve_file!("/data/footer.png", mime::IMAGE_PNG, bytes "../../data/footer.png"),
        "/data/header.png" => serve_file!("/data/header.png", mime::IMAGE_PNG, bytes "../../data/header.png"),
        "/data/title.png" => serve_file!("/data/title.png", mime::IMAGE_PNG, bytes "../../data/title.png"),

        // Catch all other requests and return a 404.
        _ => Ok(Response::from_status(StatusCode::NOT_FOUND)
            .with_body_text_plain("The page you requested could not be found\n")),
    }
}
