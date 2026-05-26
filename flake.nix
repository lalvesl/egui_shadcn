{
  description = "egui-shadcn: Shadcn UI components for egui";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Native toolchain (includes wasm32 so `cargo check --target wasm32-…` works)
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
      in
      {
        # ── Native dev shell ───────────────────────────────────────────────
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

        # ── nix run .#web — launches trunk serve ───────────────────────────
        apps.web = {
          type = "app";
          program =
            let
              script = pkgs.writeShellScript "egui-shadcn-web" ''
                set -e
                export PATH="${rustToolchain}/bin:${pkgs.trunk}/bin:$PATH"
                REPO="$(${pkgs.git}/bin/git rev-parse --show-toplevel 2>/dev/null || pwd)"
                cd "$REPO"
                echo "Starting trunk serve on http://localhost:8080 …"
                exec trunk serve --port 8080
              '';
            in
            "${script}";
        };
      }
    );
}
