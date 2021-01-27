let
  rust_overlay = import (builtins.fetchTarball https://github.com/oxalica/rust-overlay/archive/master.tar.gz);
  nixpkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
in
  with nixpkgs;
  stdenv.mkDerivation {
    name = "lr_shell";
    buildInputs = [
      z3
      rust-analyzer
      # to use a specific nighly:
      (nixpkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain)
    ];
  }
