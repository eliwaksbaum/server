{
  description = "Rocket Server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/release-22.11";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
    crane,
    ...
  } @ inputs:
    flake-utils.lib.eachSystem ["aarch64-linux" "x86_64-linux"] (system: let
      pkgs = import nixpkgs {
        inherit system;
        overlays = [rust-overlay.overlays.default];
      };
      inherit (pkgs) lib;

      rust = pkgs.rust-bin.stable.latest.default;
      craneLib = (crane.mkLib pkgs).overrideToolchain rust;

      rocket = craneLib.buildPackage {
        src = self;
        buildInputs = with pkgs; [
          openssl
          pkg-config
        ];
      };
    in {
      packages.rocket = rocket;

      devShell = pkgs.mkShell {
        name = "rocket-shell";
        inputsFrom = [rocket];
        buildInputs = with pkgs; [
          (pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" ];
          })
        ];
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
      };

      defaultPackage = rocket;

      formatter = pkgs.alejandra;
    });
}
