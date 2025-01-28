{
  description = "Rust Development";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = inputs @ { self, nixpkgs, rust-overlay, flake-utils, ... }:
  flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs {
        inherit system overlays;
      };
    in {
      devShells.default = pkgs.mkShell {
        nativeBuildInputs = [
          pkgs.clang
        ];

        buildInputs = [
          pkgs.rust-bin.stable.latest.default
          pkgs.rust-analyzer
          pkgs.godot_4
        ];

        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
      };
    }
  );
}
