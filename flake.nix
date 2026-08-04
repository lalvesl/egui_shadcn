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
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    , crane
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable."1.96.0".default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
          targets = [
            "wasm32-unknown-unknown"
            "aarch64-linux-android"
          ];
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

        # Pre-fetched icon codepoints so egui_components/build.rs doesn't need internet.
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
        # Headless browser for the Rust e2e harness (chromiumoxide drives it
        # over the DevTools Protocol — no WebDriver, no Playwright, no Python).
        e2eChromium = pkgs.chromium;

        # ── Android build tooling (cargo-apk + SDK/NDK) ────────────────────────
        # Unfree Android components need a license-accepting nixpkgs instance.
        androidPkgs = import nixpkgs {
          inherit system overlays;
          config.allowUnfree = true;
          config.android_sdk.accept_license = true;
        };
        androidComposition = androidPkgs.androidenv.composeAndroidPackages {
          platformVersions = [ "33" ];
          buildToolsVersions = [ "34.0.0" ];
          includeNDK = true;
          ndkVersions = [ "26.1.10909125" ];
        };
        androidSdkRoot = "${androidComposition.androidsdk}/libexec/android-sdk";
        androidNdkRoot = "${androidSdkRoot}/ndk/26.1.10909125";

        # ── Native build (offline) shared args ──────────────────────────────────
        # Used by the i18n catalog generator and the flake `checks`. The Nix
        # sandbox has no network, so build scripts read the pre-fetched font +
        # codepoints via env. demo/build.rs's optional Roboto/NerdFont downloads
        # fail gracefully (not needed for tests or catalog generation).
        nativeCommonArgs = {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs = nativeLibs;
          EGUI_SHADCN_CODEPOINTS_PATH = "${materialIconsCodepoints}";
          EGUI_SHADCN_FONT_PATH = "${materialIconsFont}";
          # eframe (glow/x11/wayland) is dynamically linked; resolve at run time
          # for the headless tests and the catalog-generator binary.
          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath nativeLibs;
          # Dev profile, not crane's default release: the root [profile.release]
          # sets panic="abort", which is illegal for the unwinding test harness,
          # so a release `cargo test` forks every dep into two incompatible
          # builds (E0308 "expected egui::Context, found egui::Context"). Dev is
          # also what `cargo test` / `nix run .#test` use locally. One dev deps
          # closure is then shared by the catalog gen + clippy + test.
          CARGO_PROFILE = "dev";
        };

        # Native dependency artifacts, shared by every native derivation below so
        # deps compile once and are reused (and cached in CI).
        nativeCargoArtifacts = craneLib.buildDepsOnly (
          nativeCommonArgs
          // {
            pname = "egui-shadcn-native";
            version = "0.1.0";
          }
        );

        # The native demo binary, built with every language feature so linkme
        # collects all catalogs. `--gen-i18n <dir>` exports them and exits before
        # opening a window.
        demoNative = craneLib.buildPackage (
          nativeCommonArgs
          // {
            pname = "demo-native";
            version = "0.1.0";
            cargoArtifacts = nativeCargoArtifacts;
            cargoExtraArgs = "-p demo --bin demo-native --features lang-en,lang-pt";
            doCheck = false;
          }
        );

        # The per-language i18n catalog bundles the WASM app fetches at runtime
        # from wasm_assets/i18n/{bcp47}.cat. linkme has no wasm backend, so these
        # MUST be generated from the native binary and shipped as static assets —
        # otherwise the deployed demo loads with every string missing.
        i18nCatalogs =
          pkgs.runCommand "egui-shadcn-i18n-catalogs"
            {
              buildInputs = nativeLibs;
            }
            ''
              mkdir -p "$out/i18n"
              export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath nativeLibs}"
              "${demoNative}/bin/demo-native" --gen-i18n "$out/i18n"
              echo "[i18n] generated catalogs:" && ls -l "$out/i18n"
            '';
      in
      {
        # ── Native dev shell ───────────────────────────────────────────────────
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
            trunk
            e2eChromium # headless browser for `cargo run -p e2e`
          ];

          buildInputs =
            nativeLibs
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (
              with pkgs;
              [
                darwin.apple_sdk.frameworks.SystemConfiguration
                darwin.apple_sdk.frameworks.CoreFoundation
                darwin.apple_sdk.frameworks.Security
                darwin.apple_sdk.frameworks.AppKit
                darwin.apple_sdk.frameworks.OpenGL
              ]
            );

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath nativeLibs}:$LD_LIBRARY_PATH"
            export E2E_CHROME="${e2eChromium}/bin/chromium"
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
              nativeBuildInputs = [ pkgs.pkg-config ];
              buildInputs = nativeLibs;
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
            nativeBuildInputs = [
              pkgs.pkg-config
              wasmBindgenCli
              pkgs.binaryen
              pkgs.python3
            ];
            buildInputs = nativeLibs;
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

              # i18n catalog bundles — the WASM app fetches wasm_assets/i18n/
              # {bcp47}.cat at runtime (linkme cannot embed them on wasm32).
              mkdir -p "$out/wasm_assets/i18n"
              cp ${i18nCatalogs}/i18n/*.cat "$out/wasm_assets/i18n/"

              # Minimal HTML — no trunk directives, loads WASM as ES module
              cat > "$out/index.html" <<'HTML'
              <!DOCTYPE html>
              <html lang="en">
              <head>
                <meta charset="utf-8" />
                <title>egui-shadcn demo</title>
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <link rel="icon" href="data:," />

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

        # ── nix build .#demo2android — release APK of the demo crate ──────────
        # crane vendors all deps offline, then cargo-apk cross-compiles the
        # `demo` cdylib for aarch64-linux-android under the NDK toolchain and
        # packages + signs an APK. The android entry point is `android_main`
        # in demo/src/lib.rs; android config lives in [package.metadata.android]
        # of demo/Cargo.toml.
        packages.demo2android =
          let
            src = craneLib.cleanCargoSource ./.;
          in
          craneLib.mkCargoDerivation {
            pname = "egui-shadcn-demo-android";
            version = "0.1.0";
            inherit src;
            cargoArtifacts = null;
            strictDeps = true;
            doCheck = false;
            doInstallCargoArtifacts = false;

            # egui_components/build.rs embeds the icon font on native targets
            # (android is native): hand it the pre-fetched assets so it stays offline.
            EGUI_SHADCN_CODEPOINTS_PATH = "${materialIconsCodepoints}";
            EGUI_SHADCN_FONT_PATH = "${materialIconsFont}";

            # ndk-build / cargo-apk toolchain discovery.
            ANDROID_SDK_ROOT = androidSdkRoot;
            ANDROID_NDK_ROOT = androidNdkRoot;
            JAVA_HOME = "${pkgs.jdk17_headless}";

            nativeBuildInputs = [
              pkgs.cargo-apk
              pkgs.jdk17_headless
            ];

            buildPhaseCargoCommand = ''
              # ndk-build needs a writable $HOME for its ~/.android lookup.
              export HOME=$(mktemp -d)

              # cargo-apk requires a signing key for the release profile (debug
              # auto-keys are only generated for the dev profile). Mint a throwaway
              # self-signed keystore and point the release-keystore env vars at it.
              export CARGO_APK_RELEASE_KEYSTORE="$HOME/release.keystore"
              export CARGO_APK_RELEASE_KEYSTORE_PASSWORD=android
              keytool -genkeypair -v \
                -keystore "$CARGO_APK_RELEASE_KEYSTORE" \
                -alias demo -keyalg RSA -keysize 2048 -validity 10000 \
                -storepass android -keypass android \
                -dname "CN=egui-shadcn demo, O=egui-shadcn, C=US"

              # -p demo: cargo-subcommand sees the [workspace] and needs an explicit
              # package. --lib builds only the cdylib (libdemo.so), skipping the bin.
              cargo apk build --release --lib -p demo
            '';

            installPhaseCommand = ''
              mkdir -p "$out"
              apk=$(find target -name '*.apk' ! -name '*-unaligned.apk' | head -1)
              echo "Packaged APK: $apk"
              cp "$apk" "$out/demo.apk"
            '';
          };

        packages.default = self.packages.${system}.web;
        packages.i18n-catalogs = i18nCatalogs;

        # ── nix flake check — clippy (-D warnings) + workspace tests + web build ─
        # fmt is intentionally NOT a gate: CI runs `cargo fmt --check` as a
        # non-blocking informational step so hand-tuned formatting isn't forced.
        checks = {
          web = self.packages.${system}.web;

          clippy = craneLib.cargoClippy (
            nativeCommonArgs
            // {
              pname = "egui-shadcn-clippy";
              version = "0.1.0";
              cargoArtifacts = nativeCargoArtifacts;
              cargoClippyExtraArgs = "--workspace --all-targets -- -D warnings";
            }
          );

          # Hand-rolled (not craneLib.cargoTest) to guarantee a plain dev-profile
          # `cargo test` with no injected --release — see CARGO_PROFILE note above.
          test = craneLib.mkCargoDerivation (
            nativeCommonArgs
            // {
              pname = "egui-shadcn-test";
              version = "0.1.0";
              cargoArtifacts = nativeCargoArtifacts;
              doInstallCargoArtifacts = false;
              buildPhaseCargoCommand = "cargo test --workspace";
            }
          );
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

        # ── nix run .#e2e — full confidence run (Python-free) ─────────────────
        #   1. headless unit + smoke tests — these also *run* the demo and
        #      gallery apps' real update loops (every component, every chart),
        #   2. execute the i18n example binary,
        #   3. build the WASM demo and drive it in headless Chromium via the
        #      Rust harness (chromiumoxide over CDP — replaces Playwright).
        apps.e2e = {
          type = "app";
          program =
            let
              script = pkgs.writeShellScript "egui-shadcn-e2e" ''
                set -euo pipefail
                export PATH="${rustToolchain}/bin:${pkgs.trunk}/bin:${pkgs.git}/bin:$PATH"
                export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath nativeLibs}"

                REPO="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
                cd "$REPO"

                echo "[e2e] == 1/4 unit + smoke tests (components, charts, demo, gallery, i18n) =="
                cargo test --workspace

                echo "[e2e] == 2/4 executing example app (i18n) =="
                cargo run -p example-app

                echo "[e2e] == 3/4 building WASM demo (trunk build) =="
                ( cd "$REPO/demo" && trunk build )

                echo "[e2e] == 4/4 headless browser test (Rust / chromiumoxide) =="
                export E2E_SERVE_DIR="$REPO/demo/dist"
                export E2E_CHROME="${e2eChromium}/bin/chromium"
                exec cargo run --manifest-path "$REPO/e2e/Cargo.toml" --release
              '';
            in
            "${script}";
        };

        # ── nix run .#test — headless unit + smoke suite only (fast) ──────────
        apps.test = {
          type = "app";
          program =
            let
              script = pkgs.writeShellScript "egui-shadcn-test" ''
                set -euo pipefail
                export PATH="${rustToolchain}/bin:$PATH"
                export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath nativeLibs}"
                REPO="$(${pkgs.git}/bin/git rev-parse --show-toplevel 2>/dev/null || pwd)"
                cd "$REPO"
                exec cargo test --workspace
              '';
            in
            "${script}";
        };
        apps.fmt = {
          type = "app";
          program = "${
            pkgs.writeShellApplication {
              name = "fmt";
              runtimeInputs = with pkgs; [
                rustToolchain
                nixpkgs-fmt
                taplo
                prettier
                jq
              ];
              text = ''
                cargo fmt --all
                nixpkgs-fmt .
                taplo fmt
                prettier --write "**/*.md"
                while IFS= read -r -d "" f; do
                  tmp="$(mktemp)"
                  jq . "$f" > "$tmp" && mv "$tmp" "$f"
                done < <(find . -name "*.jsonl" -not -path "./.git/*" -not -path "./target/*" -print0)
              '';
            }
          }/bin/fmt";
        };
      }
    );
}
