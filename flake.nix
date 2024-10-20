{
  description = "A flake for AVR development tools";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.pkgsBuildHost.rust-bin.fromRustupToolchainFile
          ./rust-toolchain.toml;
      in with pkgs; {
        devShells.default = mkShell {
          buildInputs = [
            rustToolchain
            avrdude
            ravedude
            usbutils
            arduino
            pkgsCross.avr.buildPackages.gcc8
          ];

          shellHook = ''
            export PATH=$PATH:${arduino}/share/arduino/hardware/tools/avr/bin/
            export RAVEDUDE_PORT=/dev/ttyUSB1
          '';
        };
      });
}
