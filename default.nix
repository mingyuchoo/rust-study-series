let
  pkgs = import <nixpkgs> {};
in
  pkgs.stdenv.mkDerivation {
    name = "rust";
    buildInputs = [
      pkgs.rustc
      pkgs.rustup
      pkgs.cargo
      #pkgs.cargo-lambda # is too low of a version to use
    ];
    shellHook = ''
      echo Welcome to nix-shell for Rust!
    '';
  }
