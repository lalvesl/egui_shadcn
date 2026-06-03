//! Minimal static file server for the e2e harness.
//!
//! Keeps the whole end-to-end flow Python-free: instead of shelling out to
//! `python3 -m http.server`, the e2e binary serves `demo/dist` itself over a
//! bare-bones HTTP/1.1 GET handler built on the tokio runtime we already use to
//! drive Chromium. Good enough to feed a WASM bundle to a browser — not a
//! general-purpose web server.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Spawn a static server for `dir` on `127.0.0.1:port`. Returns once it is bound
/// and accepting; serving continues on a background task for the process's life.
pub async fn serve(dir: PathBuf, port: u16) -> Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", port))
        .await
        .with_context(|| format!("bind 127.0.0.1:{port}"))?;
    let root = Arc::new(dir);

    tokio::spawn(async move {
        while let Ok((stream, _)) = listener.accept().await {
            let root = root.clone();
            tokio::spawn(async move {
                let _ = handle(stream, &root).await;
            });
        }
    });

    Ok(())
}

async fn handle(mut stream: TcpStream, root: &Path) -> Result<()> {
    // Read just the request head — enough to recover the GET path.
    let mut buf = [0u8; 2048];
    let n = stream.read(&mut buf).await?;
    let head = String::from_utf8_lossy(&buf[..n]);
    let path = head
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .unwrap_or("/");

    // Strip query/fragment, decode the leading slash, default to index.html.
    let rel = path.split(['?', '#']).next().unwrap_or("/");
    let rel = rel.trim_start_matches('/');
    let rel = if rel.is_empty() { "index.html" } else { rel };

    match resolve(root, rel) {
        Some(file) => {
            let body = tokio::fs::read(&file).await?;
            let ctype = content_type(&file);
            write_response(&mut stream, "200 OK", ctype, &body).await?;
        }
        None => {
            write_response(&mut stream, "404 Not Found", "text/plain", b"not found").await?;
        }
    }
    let _ = stream.shutdown().await;
    Ok(())
}

/// Resolve `rel` under `root`, rejecting any path that escapes the root.
fn resolve(root: &Path, rel: &str) -> Option<PathBuf> {
    if rel.contains("..") {
        return None;
    }
    let candidate = root.join(rel);
    candidate.is_file().then_some(candidate)
}

fn content_type(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("js") | Some("mjs") => "text/javascript; charset=utf-8",
        // Correct MIME matters: lets the browser use WebAssembly streaming.
        Some("wasm") => "application/wasm",
        Some("ttf") => "font/ttf",
        Some("css") => "text/css; charset=utf-8",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

async fn write_response(
    stream: &mut TcpStream,
    status: &str,
    content_type: &str,
    body: &[u8],
) -> Result<()> {
    let header = format!(
        "HTTP/1.1 {status}\r\n\
         Content-Type: {content_type}\r\n\
         Content-Length: {}\r\n\
         Cache-Control: no-store\r\n\
         Connection: close\r\n\r\n",
        body.len()
    );
    stream.write_all(header.as_bytes()).await?;
    stream.write_all(body).await?;
    stream.flush().await?;
    Ok(())
}
