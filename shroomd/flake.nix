{
  description = "Build shroomd";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, naersk, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        inherit (pkgs) lib;

        # Stable Rust toolchain with targets for native + RPi cross-compilation
        stableToolchain = pkgs.rust-bin.stable.latest.default.override {
          targets = [ "x86_64-unknown-linux-gnu" "aarch64-unknown-linux-gnu" "aarch64-unknown-linux-musl" ];
        };

        # Nightly Rust toolchain
        nightlyToolchain = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            targets = [ "x86_64-unknown-linux-gnu" ];
          }
        );

        # Both toolchains combined (stable binaries take priority)
        rustCombined = pkgs.buildEnv {
          name = "rust-combined";
          paths = [ stableToolchain nightlyToolchain ];
          ignoreCollisions = true;
        };

        naersk' = pkgs.callPackage naersk {
          rustc = stableToolchain;
          cargo = stableToolchain;
        };

      in {
        packages = {
          default = naersk'.buildPackage {
            src = ./.;
            nativeBuildInputs = with pkgs; [ protobuf ];
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${pkgs.protobuf}/include";
          };
        }
        // lib.optionalAttrs (system == "x86_64-linux") {
          shroomd-client-rpi = naersk'.buildPackage {
            src = ./.;
            cargoBuildOptions = old: old ++ [ "--bin" "shroomd-client" ];
            nativeBuildInputs = with pkgs; [ protobuf pkgs.pkgsCross.aarch64-multiplatform-musl.stdenv.cc ];
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            PROTOC_INCLUDE = "${pkgs.protobuf}/include";
            CARGO_BUILD_TARGET = "aarch64-unknown-linux-musl";
            CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = "${pkgs.pkgsCross.aarch64-multiplatform-musl.stdenv.cc.targetPrefix}cc";
          };
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            rustCombined
            protobuf
          ] ++ lib.optionals (system == "x86_64-linux") [
            pkgs.pkgsCross.aarch64-multiplatform-musl.stdenv.cc
          ];

          PROTOC = "${pkgs.protobuf}/bin/protoc";
          PROTOC_INCLUDE = "${pkgs.protobuf}/include";
          CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER = lib.optionalString (system == "x86_64-linux")
            "${pkgs.pkgsCross.aarch64-multiplatform-musl.stdenv.cc.targetPrefix}cc";
        };
      }
    );
}
