//! End-to-end browser test for the egui-shadcn WASM demo.
//!
//! Rust port of the former `e2e/test.py` (Playwright). Drives a headless
//! Chromium over the DevTools Protocol via `chromiumoxide` — the closest
//! Rust analogue to Playwright: async, event-driven, no extra WebDriver.
//!
//! Run via: `nix run .#e2e` (which builds the WASM, serves it, then runs this).
//!
//! Config via env:
//!   E2E_URL    target URL              (default http://localhost:8081)
//!   E2E_CHROME path to chrome/chromium (default: auto-detected on PATH)
//!   E2E_WAIT   render wait, seconds    (default 6)

use std::sync::{Arc, Mutex};
use std::time::Duration;

use anyhow::{bail, Context as _, Result};
use chromiumoxide::cdp::browser_protocol::log::{EventEntryAdded, LogEntryLevel};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use chromiumoxide::cdp::js_protocol::runtime::{EventConsoleApiCalled, EventExceptionThrown};
use chromiumoxide::page::ScreenshotParams;
use chromiumoxide::{Browser, BrowserConfig};
use futures::StreamExt;

mod server;

/// Substrings that mark a fatal failure if they appear in the browser console.
const CRASH_KEYWORDS: &[&str] = &[
    "MaterialIcons",
    "TTF/OTF",
    "offset was out of bounds",
    "panicked at",
    "RuntimeError: Unreachable",
    "WebAssembly.instantiate",
    "wasm trap",
];

/// Page background `#09090b` = rgb(9, 9, 11).
const BG_COLOR: [u8; 3] = [9, 9, 11];

/// Minimum number of non-background pixels for the canvas to count as "rendered".
const MIN_NON_BG_PIXELS: u64 = 1000;

/// Minimum number of distinct colors expected from a real rendered UI. A blank
/// or lost-context canvas is a single solid fill (≈1 color) — which would still
/// pass the non-background check — so this guards against that false positive.
const MIN_DISTINCT_COLORS: usize = 12;

#[derive(Clone, Default)]
struct Collected {
    /// `"[type] text"` for every console message, in order.
    messages: Arc<Mutex<Vec<String>>>,
    /// Only the messages classed as errors (console.error + thrown exceptions).
    errors: Arc<Mutex<Vec<String>>>,
}

impl Collected {
    fn push_msg(&self, kind: &str, text: String) {
        self.messages.lock().unwrap().push(format!("[{kind}] {text}"));
    }
    fn push_err(&self, text: String) {
        self.errors.lock().unwrap().push(text);
    }
    fn snapshot(&self) -> (Vec<String>, Vec<String>) {
        (
            self.messages.lock().unwrap().clone(),
            self.errors.lock().unwrap().clone(),
        )
    }
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("[e2e] FAIL — {e:#}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let port: u16 = std::env::var("E2E_PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8081);
    let url = std::env::var("E2E_URL").unwrap_or_else(|_| format!("http://127.0.0.1:{port}/"));
    let wait_secs: u64 = std::env::var("E2E_WAIT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(6);

    // Self-serve a static bundle (e.g. demo/dist) so the whole flow is Python-free.
    if let Ok(dir) = std::env::var("E2E_SERVE_DIR") {
        let dir = std::path::PathBuf::from(&dir);
        anyhow::ensure!(dir.is_dir(), "E2E_SERVE_DIR is not a directory: {dir:?}");
        server::serve(dir.clone(), port).await?;
        println!("[e2e] Serving {} on :{port}", dir.display());
    }

    // ── Launch headless Chromium ─────────────────────────────────────────────
    let mut builder = BrowserConfig::builder()
        .no_sandbox()
        .arg("--disable-dev-shm-usage")
        // WebGL via SwiftShader (software renderer) so egui's glow backend works
        // in headless mode without a GPU. Recent Chromium gates the software
        // fallback behind --enable-unsafe-swiftshader (otherwise the context is
        // immediately lost), so opt in explicitly.
        .arg("--use-gl=angle")
        .arg("--use-angle=swiftshader")
        .arg("--enable-unsafe-swiftshader")
        .arg("--enable-webgl")
        .arg("--ignore-gpu-blocklist")
        .arg("--window-size=1280,720")
        .arg("--hide-scrollbars");
    if let Ok(path) = std::env::var("E2E_CHROME") {
        builder = builder.chrome_executable(path);
    }
    let config = builder
        .build()
        .map_err(|e| anyhow::anyhow!("BrowserConfig: {e}"))?;

    let (browser, mut handler) = Browser::launch(config)
        .await
        .context("failed to launch Chromium (set E2E_CHROME to a chromium binary)")?;

    // The handler future drives the CDP connection; it must be polled for the
    // life of the session. Stop when the browser closes.
    let handler_task = tokio::spawn(async move { while handler.next().await.is_some() {} });

    let collected = Collected::default();
    let page = browser.new_page("about:blank").await?;

    // Surface load-domain entries (network/security/rendering warnings + errors).
    let _ = page.execute(chromiumoxide::cdp::browser_protocol::log::EnableParams::default()).await;

    // ── Console + exception listeners ────────────────────────────────────────
    spawn_console_listener(&page, collected.clone()).await?;
    spawn_exception_listener(&page, collected.clone()).await?;
    spawn_log_listener(&page, collected.clone()).await?;

    // ── Navigate + wait for WASM/font/egui first render ──────────────────────
    println!("[e2e] Loading {url} ...");
    page.goto(&url).await.context("navigation failed")?;
    page.wait_for_navigation().await.ok();
    tokio::time::sleep(Duration::from_secs(wait_secs)).await;

    let (messages, errors) = collected.snapshot();
    if !messages.is_empty() {
        let tail = messages.iter().rev().take(5).collect::<Vec<_>>();
        println!("[e2e] Console ({} msgs, last 5):", messages.len());
        for m in tail.into_iter().rev() {
            println!("  {m}");
        }
    }

    // ── Check 1: no crash-signature errors in console ────────────────────────
    let fatal: Vec<&String> = errors
        .iter()
        .filter(|e| CRASH_KEYWORDS.iter().any(|kw| e.contains(kw)))
        .collect();
    if !fatal.is_empty() {
        eprintln!("[e2e] fatal errors in browser console:");
        for e in &fatal {
            eprintln!("  {e}");
        }
        cleanup(browser, handler_task).await;
        bail!("{} fatal console error(s)", fatal.len());
    }

    // ── Check 2: canvas rendered non-background pixels ───────────────────────
    // egui renders via WebGL, so a 2d context is null. A CDP screenshot captures
    // the composited frame regardless of backend (same trick the Python test used).
    let png = page
        .screenshot(
            ScreenshotParams::builder()
                .format(CaptureScreenshotFormat::Png)
                .full_page(false)
                .build(),
        )
        .await
        .context("screenshot failed")?;

    let img = image::load_from_memory(&png)
        .context("decode screenshot PNG")?
        .to_rgb8();
    let (w, h) = (img.width(), img.height());
    let total = (w as u64) * (h as u64);
    let mut non_bg = 0u64;
    let mut colors = std::collections::HashSet::new();
    for p in img.pixels() {
        if p.0 != BG_COLOR {
            non_bg += 1;
        }
        // Quantize to 4 bits/channel so anti-aliasing doesn't inflate the count.
        colors.insert([p.0[0] >> 4, p.0[1] >> 4, p.0[2] >> 4]);
    }
    let distinct = colors.len();
    let pct = (100 * non_bg).checked_div(total).unwrap_or(0);
    println!(
        "[e2e] Screenshot: {w}x{h}, non-background pixels: {non_bg}/{total} ({pct}%), distinct colors: {distinct}"
    );

    if non_bg < MIN_NON_BG_PIXELS {
        cleanup(browser, handler_task).await;
        bail!("canvas appears blank or black (egui may not have rendered)");
    }
    if distinct < MIN_DISTINCT_COLORS {
        cleanup(browser, handler_task).await;
        bail!(
            "canvas has only {distinct} distinct color(s) — looks like a solid fill, \
             not a rendered UI (WebGL context lost?)"
        );
    }

    println!("[e2e] PASS — app loaded, no font errors, canvas rendered a real UI");
    cleanup(browser, handler_task).await;
    Ok(())
}

async fn cleanup(mut browser: Browser, handler_task: tokio::task::JoinHandle<()>) {
    let _ = browser.close().await;
    let _ = browser.wait().await;
    handler_task.abort();
}

async fn spawn_console_listener(page: &chromiumoxide::Page, c: Collected) -> Result<()> {
    let mut events = page.event_listener::<EventConsoleApiCalled>().await?;
    tokio::spawn(async move {
        while let Some(ev) = events.next().await {
            let kind = format!("{:?}", ev.r#type).to_lowercase();
            let text = ev
                .args
                .iter()
                .map(remote_object_text)
                .collect::<Vec<_>>()
                .join(" ");
            c.push_msg(&kind, text.clone());
            if kind.contains("error") {
                c.push_err(text);
            }
        }
    });
    Ok(())
}

async fn spawn_exception_listener(page: &chromiumoxide::Page, c: Collected) -> Result<()> {
    let mut events = page.event_listener::<EventExceptionThrown>().await?;
    tokio::spawn(async move {
        while let Some(ev) = events.next().await {
            let d = &ev.exception_details;
            let text = d
                .exception
                .as_ref()
                .and_then(remote_object_text_opt)
                .unwrap_or_else(|| d.text.clone());
            c.push_msg("pageerror", text.clone());
            c.push_err(text);
        }
    });
    Ok(())
}

async fn spawn_log_listener(page: &chromiumoxide::Page, c: Collected) -> Result<()> {
    let mut events = page.event_listener::<EventEntryAdded>().await?;
    tokio::spawn(async move {
        while let Some(ev) = events.next().await {
            let entry = &ev.entry;
            let text = entry.text.clone();
            c.push_msg("log", text.clone());
            if matches!(entry.level, LogEntryLevel::Error) {
                c.push_err(text);
            }
        }
    });
    Ok(())
}

/// Best-effort human text for a CDP RemoteObject (console arg).
fn remote_object_text(o: &chromiumoxide::cdp::js_protocol::runtime::RemoteObject) -> String {
    remote_object_text_opt(o).unwrap_or_default()
}

fn remote_object_text_opt(
    o: &chromiumoxide::cdp::js_protocol::runtime::RemoteObject,
) -> Option<String> {
    if let Some(v) = &o.value {
        // Strings come through as JSON strings; render them unquoted.
        return Some(match v {
            serde_json::Value::String(s) => s.clone(),
            other => other.to_string(),
        });
    }
    o.description.clone()
}
