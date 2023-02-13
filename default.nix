let
  pkgs = import <nixpkgs> {};
in
  pkgs.stdenv.mkDerivation {
    name = "rust";
    buildInputs = [
      pkgs.cargo
      pkgs.rustc
    ];
    shellHook = ''
      Welcome to nix-shell for Rust!
    '';
  }
