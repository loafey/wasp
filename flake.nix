{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { flake-utils, nixpkgs, naersk, fenix, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ fenix.overlays.default ];
        pkgs = import nixpkgs { inherit system overlays; };
        toolchain = with fenix.packages.${system};
          combine [
            minimal.cargo
            minimal.rustc
            latest.clippy
            latest.rust-src
            latest.rustfmt
            targets.wasm32-unknown-unknown.latest.rust-std
            targets.x86_64-unknown-linux-gnu.latest.rust-std
          ];

        fetchy = (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ ] ++ min-pkgs;
        };

        updated-wabt = pkgs.wabt.overrideAttrs (finalAttrs: previousAttrs: {
          src = pkgs.fetchFromGitHub {
            "owner" = "WebAssembly";
            "repo" = "wabt";
            "rev" = "6be9e863ce5766a9e3fc1be8a5b2a8bbb7b37bb0";
            "hash" = "sha256-72oHUzRAqm2lFLKHNivPVOLyifix1Z79jRy17EPuUsI=";
            "fetchSubmodules" = true;
          };
        });
        min-pkgs = [
          pkgs.pkg-config
          pkgs.openssl
          pkgs.gcc
          pkgs.emscripten
          pkgs.gnumake
          pkgs.python3
          updated-wabt
          pkgs.watchexec
          pkgs.gettext
          pkgs.pandoc
        ];
      in {
        defaultPackage = fetchy;

        packages = { fetchy = fetchy; };

        devShell = (naersk.lib.${system}.override {
          cargo = toolchain;
          rustc = toolchain;
        }).buildPackage {
          src = ./.;
          nativeBuildInputs = with pkgs; [ ] ++ min-pkgs;

          shellHook = ''
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${
              pkgs.lib.makeLibraryPath min-pkgs
            }"'';
        };
      });
}
