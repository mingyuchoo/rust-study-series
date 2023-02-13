let
  pkgs = import <nixpkgs> {};
in
  pkgs.stdenv.mkDerivation {
    name = "rust";
    buildInputs = [
      pkgs.rustc
      pkgs.cargo
      pkgs.cargo-lambda
    ];
    shellHook = ''
      echo Welcome to nix-shell for Rust!
    '';
  }
