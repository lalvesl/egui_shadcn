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

        rustToolchain = pkgs.rust-bin.stable."1.94.1".default.override {
          extensions = [
            "rust-src"
            "rust-analyzer"
            "clippy"
          ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            pkg-config
            rustToolchain
          ];

          buildInputs =
            with pkgs;
            [
              # egui/eframe on Linux (X11 + Wayland + OpenGL)
              libxkbcommon
              libGL
              wayland
              xorg.libX11
              xorg.libXcursor
              xorg.libXrandr
              xorg.libXi
              fontconfig
            ]
            ++ lib.optionals stdenv.isDarwin [
              darwin.apple_sdk.frameworks.SystemConfiguration
              darwin.apple_sdk.frameworks.CoreFoundation
              darwin.apple_sdk.frameworks.Security
              darwin.apple_sdk.frameworks.AppKit
              darwin.apple_sdk.frameworks.OpenGL
            ];

          shellHook = ''
            export LD_LIBRARY_PATH="${pkgs.lib.makeLibraryPath [
              pkgs.libxkbcommon
              pkgs.libGL
              pkgs.wayland
              pkgs.xorg.libX11
              pkgs.xorg.libXcursor
              pkgs.xorg.libXrandr
              pkgs.xorg.libXi
              pkgs.fontconfig
            ]}:$LD_LIBRARY_PATH"
          '';
        };
      }
    );
}
