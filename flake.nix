{
  description = "egui-shadcn: Shadcn UI components for egui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      crane,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable."1.95.0".default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
          targets = [ "wasm32-unknown-unknown" ];
        };

        nativeLibs = with pkgs; [
          libxkbcommon
          libGL
          wayland
          libx11
          libxcursor
          libxrandr
          libxi
          fontconfig
          openssl
        ];

        # ── WASM build tooling ─────────────────────────────────────────────────
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustToolchain;
          rustc = rustToolchain;
        };

        # Pre-fetched MaterialIcons font (raw) for the Nix sandbox build.
        # Hash of: https://github.com/google/material-design-icons/raw/master/font/MaterialIcons-Regular.ttf
        materialIconsFont = pkgs.fetchurl {
          url = "https://github.com/google/material-design-icons/raw/master/font/MaterialIcons-Regular.ttf";
          hash = "sha256-7xSfCL3S/wmk4shXNHa3sPP7sVtiOVSt5ZiZ5xdb7do=";
        };

        # Pre-fetched icon codepoints so egui-shadcn/build.rs doesn't need internet.
        # Hash of: https://raw.githubusercontent.com/google/material-design-icons/master/font/MaterialIcons-Regular.codepoints
        materialIconsCodepoints = pkgs.fetchurl {
          url = "https://raw.githubusercontent.com/google/material-design-icons/master/font/MaterialIcons-Regular.codepoints";
          hash = "sha256-Uw8lv3stccjh2pR21T+am7a34Ye/9pu3Eou2ebgZSJQ=";
        };

        # wasm-bindgen-cli at the version matching Cargo.lock (0.2.122).
        # nixpkgs ships 0.2.121; a mismatch causes "schema version" errors.
        wasmBindgenSrc = pkgs.fetchurl {
          url = "https://static.crates.io/crates/wasm-bindgen-cli/wasm-bindgen-cli-0.2.122.crate";
          name = "wasm-bindgen-cli-0.2.122.tar.gz";
          hash = "sha256-wWhvn+A4+EuJLBDTt0ibKR6xEFN0UBWeuX5fhGswRbw=";
        };
        wasmBindgenCli = rustPlatform.buildRustPackage {
          pname = "wasm-bindgen-cli";
          version = "0.2.122";
          src = wasmBindgenSrc;
          # cargoHash uses fetchCargoVendor (runs `cargo vendor` in an FOD)
          # which sends User-Agent: cargo/X.Y.Z — avoids crates.io API 403.
          # cargoLock/importCargoLock would use Nix fetchers (User-Agent: Nix)
          # which crates.io now rejects.
          cargoHash = "sha256-Inup6vvJSG5ghNyeDPyZbfZo4d0LsMG2OJfStoaeDBs=";
          doCheck = false;
        };

        # crane: compiles only crates needed for the specified target.
        # Unlike importCargoLock, it won't try to build android-activity
        # (gated cfg(target_os="android")) when target is wasm32-unknown-unknown.
        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # ── e2e tooling ────────────────────────────────────────────────────────
        e2ePython = pkgs.python3.withPackages (ps: [ ps.playwright ps.pillow ]);
      in
      {
        # ── Native dev shell ───────────────────────────────────────────────────
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
            trunk
          ];

          buildInputs = nativeLibs ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [
            darwin.apple_sdk.frameworks.SystemConfiguration
            darwin.apple_sdk.frameworks.CoreFoundation
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.AppKit
            darwin.apple_sdk.frameworks.OpenGL
          ]);

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath nativeLibs}:$LD_LIBRARY_PATH"
          '';
        };

        # ── nix build .#web — optimized WASM release build ────────────────────
        # crane compiles only crates needed for wasm32-unknown-unknown target,
        # so android-activity (cfg(target_os="android")) is never built.
        packages.web =
          let
            src = craneLib.cleanCargoSource ./.;
            cargoArtifacts = craneLib.buildDepsOnly {
              inherit src;
              pname = "egui-shadcn-web";
              version = "0.1.0";
              strictDeps = true;
              CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
              cargoExtraArgs = "--target wasm32-unknown-unknown";
              EGUI_SHADCN_CODEPOINTS_PATH = "${materialIconsCodepoints}";
              doCheck = false;
            };
          in
          craneLib.mkCargoDerivation {
            pname = "egui-shadcn-web";
            version = "0.1.0";
            inherit src cargoArtifacts;
            strictDeps = true;
            CARGO_BUILD_TARGET = "wasm32-unknown-unknown";
            EGUI_SHADCN_CODEPOINTS_PATH = "${materialIconsCodepoints}";
            buildPhaseCargoCommand = "cargo build --release --target wasm32-unknown-unknown -p demo --lib";
            nativeBuildInputs = [ wasmBindgenCli pkgs.binaryen pkgs.python3 ];
            doInstallCargoArtifacts = false;
            installPhase = ''
              mkdir -p "$out/wasm_assets"

              # Generate JS bindings
              wasm-bindgen \
                --out-dir "$out" \
                --target web \
                --no-typescript \
                target/wasm32-unknown-unknown/release/demo.wasm

              # Optimize WASM: shrink size, strip debug/producer metadata
              wasm-opt -Oz \
                --enable-bulk-memory \
                --enable-nontrapping-float-to-int \
                --enable-sign-ext \
                --strip-debug \
                --strip-producers \
                --dce \
                --merge-blocks \
                --optimize-instructions \
                --output "$out/demo_bg.wasm" \
                "$out/demo_bg.wasm"

              # Strip GPOS/GSUB from font so skrifa doesn't crash on wasm32,
              # then serve it from wasm_assets/ (fetched by the app at runtime).
              python3 -c "
data = bytearray(open('${materialIconsFont}', 'rb').read())
n = (data[4] << 8) | data[5]
for i in range(n):
    b = 12 + i * 16
    if b + 4 > len(data): break
    if bytes(data[b:b+4]) in (b'GPOS', b'GSUB'): data[b] = ord('X')
open('$out/wasm_assets/MaterialIcons-Regular.ttf', 'wb').write(bytes(data))
"

              # Minimal HTML — no trunk directives, loads WASM as ES module
              cat > "$out/index.html" <<'HTML'
<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <title>egui-shadcn demo</title>
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <style>
    html, body { margin: 0; padding: 0; overflow: hidden; width: 100%; height: 100%; background: #09090b; }
    canvas { display: block; width: 100% !important; height: 100% !important; }
  </style>
</head>
<body>
  <canvas id="egui_canvas"></canvas>
  <script type="module">
    import init from './demo.js';
    init();
  </script>
</body>
</html>
HTML
            '';
            doCheck = false;
          };

        # ── nix run .#web — trunk serve (dev) ─────────────────────────────────
        apps.web = {
          type = "app";
          program =
            let
              script = pkgs.writeShellScript "egui-shadcn-web" ''
                set -e
                export PATH="${rustToolchain}/bin:${pkgs.trunk}/bin:$PATH"
                REPO="$(${pkgs.git}/bin/git rev-parse --show-toplevel 2>/dev/null || pwd)"
                cd "$REPO/demo"
                echo "Starting trunk serve on http://localhost:8080 …"
                exec trunk serve --port 8080
              '';
            in
            "${script}";
        };

        # ── nix run .#e2e — end-to-end browser test ───────────────────────────
        apps.e2e = {
          type = "app";
          program =
            let
              script = pkgs.writeShellScript "egui-shadcn-e2e" ''
                set -euo pipefail
                export PATH="${rustToolchain}/bin:${pkgs.trunk}/bin:${pkgs.git}/bin:$PATH"
                export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath nativeLibs}"
                export PLAYWRIGHT_BROWSERS_PATH="${pkgs.playwright-driver.browsers}"
                export PLAYWRIGHT_SKIP_BROWSER_DOWNLOAD=1

                REPO="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"

                echo "[e2e] Building WASM with trunk…"
                (cd "$REPO/demo" && trunk build)

                echo "[e2e] Serving demo/dist on :8081…"
                ${pkgs.python3}/bin/python3 -m http.server 8081 \
                  --directory "$REPO/demo/dist" &
                SERVER_PID=$!
                trap "kill $SERVER_PID 2>/dev/null || true" EXIT

                echo "[e2e] Waiting for server to be ready…"
                ${pkgs.python3}/bin/python3 -c "
import urllib.request, time, sys
for _ in range(40):
    try:
        urllib.request.urlopen('http://localhost:8081/')
        break
    except Exception:
        time.sleep(0.25)
else:
    print('[e2e] ERROR: HTTP server did not start', file=sys.stderr)
    sys.exit(1)
"

                echo "[e2e] Running Playwright test…"
                ${e2ePython}/bin/python3 "$REPO/e2e/test.py"
              '';
            in
            "${script}";
        };
      }
    );
}
