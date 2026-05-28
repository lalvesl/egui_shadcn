"""
End-to-end browser test for the egui-shadcn WASM demo.
Run via: nix run .#e2e
"""

import asyncio
import io
import sys
from PIL import Image
from playwright.async_api import async_playwright


CRASH_KEYWORDS = [
    "MaterialIcons",
    "TTF/OTF",
    "offset was out of bounds",
    "panicked at",
    "RuntimeError: Unreachable",
    "WebAssembly.instantiate",
    "wasm trap",
]

# Page background: #09090b = rgb(9, 9, 11)
BG_COLOR = (9, 9, 11)


async def run() -> None:
    async with async_playwright() as p:
        browser = await p.chromium.launch(
            headless=True,
            args=[
                "--no-sandbox",
                "--disable-dev-shm-usage",
                # WebGL via SwiftShader (software renderer) for headless mode
                "--use-gl=angle",
                "--use-angle=swiftshader-webgl",
                "--enable-webgl",
            ],
        )
        page = await browser.new_page(viewport={"width": 1280, "height": 720})

        console_msgs: list[str] = []
        console_errors: list[str] = []

        def on_console(msg):
            console_msgs.append(f"[{msg.type}] {msg.text}")
            if msg.type == "error":
                console_errors.append(msg.text)

        page.on("console", on_console)
        page.on("pageerror", lambda e: console_errors.append(str(e)))

        print("[e2e] Loading http://localhost:8081 ...")
        await page.goto("http://localhost:8081", timeout=20_000)

        # Wait for WASM load + font fetch + egui initialization + first render
        await asyncio.sleep(6)

        if console_msgs:
            print(f"[e2e] Console ({len(console_msgs)} msgs, last 5):")
            for m in console_msgs[-5:]:
                print(f"  {m}")

        # --- Check 1: no crash-signature errors in console ---
        fatal = [e for e in console_errors if any(kw in e for kw in CRASH_KEYWORDS)]
        if fatal:
            print("[e2e] FAIL — fatal errors in browser console:")
            for e in fatal:
                print(f"  {e}")
            await browser.close()
            sys.exit(1)

        # --- Check 2: canvas rendered non-background pixels ---
        # egui renders via WebGL so getContext("2d") returns null.
        # Take a Playwright screenshot instead — it captures the composited frame.
        shot_bytes = await page.screenshot(full_page=False)
        img = Image.open(io.BytesIO(shot_bytes)).convert("RGB")
        pixels = list(img.getdata())
        non_bg = sum(1 for px in pixels if px != BG_COLOR)
        total = len(pixels)

        print(f"[e2e] Screenshot: {img.width}x{img.height}, "
              f"non-background pixels: {non_bg}/{total} "
              f"({100*non_bg//total}%)")

        if non_bg < 1000:
            print("[e2e] FAIL — canvas appears blank or black (egui may not have rendered)")
            await browser.close()
            sys.exit(1)

        print("[e2e] PASS — app loaded, no font errors, canvas has content")
        await browser.close()


asyncio.run(run())
