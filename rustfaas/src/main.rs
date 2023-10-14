use hyper::{service::make_service_fn, service::service_fn, Body, Request, Response, Server};
use hyper::http::StatusCode;
use std::convert::Infallible;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use hyper::header::CONTENT_TYPE;
use libloading::{Library, Symbol};

type Func = unsafe fn() -> i32; // Adjust the function signature accordingly

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&hyper::Method::POST, "/upload") => handle_upload(req).await,
        (&hyper::Method::GET, path) if path.starts_with("/invoke/") => handle_invoke(req).await,
        _ => Ok(Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Not found"))
            .unwrap()),
    }
}

async fn handle_upload(mut req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let headers = req.headers().clone();

    if let Some(content_type) = headers.get(CONTENT_TYPE) {
        if let Ok(ct) = std::str::from_utf8(content_type.as_bytes()) {
            if ct.starts_with("multipart/form-data") {
                let boundary = ct.split("boundary=").collect::<Vec<_>>().get(1).cloned().unwrap_or_default();
                let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap();

                let parts = body_bytes.split(|&b| b == b'\n')
                    .filter(|line| !line.starts_with(b"--") && !line.is_empty() && line != &boundary.as_bytes())
                    .collect::<Vec<_>>();

                // Rudimentary parsing, assuming every two slices are headers and content
                for i in (0..parts.len()).step_by(2) {
                    let headers = parts.get(i);
                    let content = parts.get(i + 1);

                    if let (Some(headers), Some(content)) = (headers, content) {
                        if headers.starts_with(b"Content-Disposition") {
                            // Extract filename and write content to a file
                            // This is a simple example; in a real-world scenario, you'll need more comprehensive parsing
                            if let Some(start) = headers.windows(b"filename=\"".len()).position(|w| w == b"filename=\"") {
                                let filename_start = start + b"filename=\"".len();
                                let filename_end = headers[filename_start..].iter().position(|&b| b == b'"').unwrap_or(0) + filename_start;
                                let filename = &headers[filename_start..filename_end];
                                let file_path = format!("./uploads/{}", std::str::from_utf8(filename).unwrap());

                                tokio::fs::write(&file_path, content).await.unwrap();
                            }
                        }
                    }
                }

                return Ok(Response::new(Body::from("File uploaded successfully")));
            }
        }
    }
    Ok(Response::new(Body::from("Invalid request")))
}

async fn handle_invoke(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    // Parse the path to get the function name
    let path = req.uri().path();
    let parts: Vec<&str> = path.split('/').collect();
    if parts.len() < 3 {
        return Ok(Response::builder().status(StatusCode::BAD_REQUEST).body(Body::from("Invalid path")).unwrap());
    }
    let function_name = parts[2];

    // Construct the path to the uploaded library
    let lib_path = format!("uploads/{}.so", function_name); // Assuming a Unix-like system; adjust extension if necessary

    if !std::path::Path::new(&lib_path).exists() {
        return Ok(Response::builder().status(StatusCode::NOT_FOUND).body(Body::from("Library not found")).unwrap());
    }

    // Load the library
    let lib = unsafe { Library::new(&lib_path) }.expect("Failed to load library");

    unsafe {
        // Load the function symbol from the library
        let function_name_bytes = function_name.as_bytes();
        let func: Symbol<Func> = lib.get(function_name_bytes).expect("Failed to get symbol"); // Replace "my_function" with your function's name

        // Call the function
        let result = func();

        Ok(Response::new(Body::from(format!("Function returned: {}", result))))
    }
}

#[tokio::main]
async fn main() {
    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let addr = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    println!("Server started on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
