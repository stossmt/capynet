use crate::error::CapyError;
use std::io::{Read, Write};
use std::net::{TcpStream, ToSocketAddrs};

pub fn fetch_url(url: &str) -> Result<String, CapyError> {
    // Parse the URL
    let (host, path) = parse_url(url)?;

    // Resolve DNS
    let addr = format!("{}:80", host);
    let addrs: Vec<_> = addr.to_socket_addrs()?.collect();
    if addrs.is_empty() {
        return Err(CapyError::new(
            crate::error::ErrorCode::InvalidArgument,
            "Could not resolve address",
        ));
    }

    // Connect to the server
    let mut stream = TcpStream::connect(addrs[0])?;

    // Send HTTP GET request
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, host
    );
    stream.write_all(request.as_bytes())?;

    // Read the response
    let mut response = String::new();
    stream.read_to_string(&mut response)?;

    // Separate headers from body
    let body = response
        .split("\r\n\r\n")
        .nth(1)
        .ok_or(CapyError::new(
            crate::error::ErrorCode::InvalidArgument,
            "TODO: add message",
        ))?
        .to_string();
    Ok(body)
}

fn parse_url(url: &str) -> Result<(&str, String), CapyError> {
    if url.starts_with("http://") {
        let url = &url[7..]; // strip "http://"
        if let Some((host, path)) = url.split_once('/') {
            Ok((host, format!("/{}", path)))
        } else {
            Ok((url, String::from("/")))
        }
    } else {
        Err(CapyError::new(
            crate::error::ErrorCode::InvalidArgument,
            "Only HTTP URLs are supported",
        ))
    }
}
