{
  description = "ShroomApp dev shell";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        androidSdk = "/home/chuu/Android/Sdk";
        jdk = pkgs.jdk17;
      in
      {
        devShells.default = pkgs.mkShell {
          JAVA_HOME = jdk;
          ANDROID_HOME = androidSdk;
          ANDROID_SDK_ROOT = androidSdk;
          GRADLE_OPTS = "-Dorg.gradle.java.home=${jdk}";

          shellHook = ''
            export PATH="${androidSdk}/platform-tools:${androidSdk}/cmdline-tools/bin:${androidSdk}/build-tools/36.0.0:$PATH"
          '';
        };
      }
    );
}
