{
  description = "A Nix-flake-based Rust development environment";

  inputs = {
    nixpkgs.url = "https://flakehub.com/f/NixOS/nixpkgs/0.1"; # unstable Nixpkgs
    fenix = {
      url = "https://flakehub.com/f/nix-community/fenix/0.1";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    { self, ... }@inputs:

    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "aarch64-darwin"
      ];
      forEachSupportedSystem =
        f:
        inputs.nixpkgs.lib.genAttrs supportedSystems (
          system:
          f {
            inherit system;
            pkgs = import inputs.nixpkgs {
              inherit system;
              overlays = [
                inputs.self.overlays.default
                inputs.rust-overlay.overlays.default
              ];
            };
          }
        );
    in
    {
      overlays.default = final: prev: {
        rustToolchain =
          with inputs.fenix.packages.${prev.stdenv.hostPlatform.system};
          combine (
            with stable;
            [
              clippy
              rustc
              cargo
              rustfmt
              rust-src
            ]
          );
      };

      devShells = forEachSupportedSystem (
        { pkgs, system }:
        let
            gui_pkgs = with pkgs; [
              libGL
              libxkbcommon
              wayland
              libX11
              libXcursor
              libXi
              libXrandr
              fontconfig
              slint-lsp
              pango
              gtk3
              glib
              gdk-pixbuf
              atk
              freetype
            ];
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              rustToolchain
              openssl
              pkg-config
              fontconfig
              cargo-deny
              cargo-edit
              cargo-watch
              rust-analyzer

              slint-tr-extractor
              poedit

              self.formatter.${system}
            ] ++ gui_pkgs;

            env = {
              # Required by rust-analyzer
              RUST_SRC_PATH = "${pkgs.rustToolchain}/lib/rustlib/src/rust/library";

              LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath gui_pkgs;
            };
          };
          windows = let
            crossPkgs = pkgs.pkgsCross.mingwW64;
            rustToolchain = pkgs.rust-bin.stable.latest.default.override {
                targets = [ "x86_64-pc-windows-gnu" ];
            };
          in pkgs.mkShell {
            buildInputs = [ rustToolchain crossPkgs.stdenv.cc crossPkgs.windows.pthreads ];

            CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER =
                "${crossPkgs.stdenv.cc}/bin/${crossPkgs.stdenv.cc.targetPrefix}cc";
          };
        }
      );

      formatter = forEachSupportedSystem ({ pkgs, ... }: pkgs.nixfmt);
    };
}
