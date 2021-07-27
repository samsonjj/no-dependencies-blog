extern crate http;

use http::*;
use std::path::PathBuf;
use std::fs;
use std::collections::HashSet;

fn not_found_response() -> HttpResponse {
    let not_found_body = b"<h1>404 Not Found</h1>".to_vec();
    HttpResponse::new(
        HttpVersion::default(),
        HttpStatusCode(404),
        HttpHeaders::default(),
        Some(not_found_body)
    )
}

fn forbidden_response() -> HttpResponse {
    let not_found_body = b"<h1>403 Forbidden</h1>".to_vec();
    HttpResponse::new(
        HttpVersion::default(),
        HttpStatusCode(403),
        HttpHeaders::default(),
        Some(not_found_body)
    )
}

fn main() -> std::io::Result<()> {
    let mut server = HttpServer::new();

    // for static page path validation (preventing path manipulation vulnerabilities)
    let valid_pages: HashSet<PathBuf> = fs::read_dir("./pages").unwrap()
        .map(|p| p.unwrap().path())
        .collect();

    server.request_handler = std::sync::Arc::new(move |req: HttpRequest| -> HttpResponse {
        let body = b"<h1>Big boy time</h1>".to_vec();

        println!("path: {:?}", req.uri);
        println!("{:?}", PathBuf::from("/static/../Cargo.toml"));
        if req.uri == PathBuf::new().join("/") {
            HttpResponse::new(
                HttpVersion::default(),
                HttpStatusCode(200),
                HttpHeaders::default(),
                Some(body)
            )
        } else if req.uri.starts_with(PathBuf::new().join("/static")) {
            let static_file_path = PathBuf::new()
                .join("pages")
                .join(req.uri.iter().skip(2).collect::<PathBuf>());
            if !valid_pages.contains(&static_file_path) {
                return forbidden_response()
            }
            println!("path: {:?}", static_file_path);
            let data = match fs::read(static_file_path) {
                Ok(data) => data,
                _ => return not_found_response()
            };
            HttpResponse::new(
                HttpVersion::default(),
                HttpStatusCode(200),
                HttpHeaders::default(),
                Some(data)
            )
        } else {
            not_found_response()
        }
    });

    server.listen(8080).unwrap();

    Ok(())
}