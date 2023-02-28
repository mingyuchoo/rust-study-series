let
  pkgs = import <nixpkgs> {};
in
  pkgs.stdenv.mkDerivation {
    name = "rust";
    buildInputs = [
      pkgs.direnv
      pkgs.cargo
      pkgs.rustc
      pkgs.rustup
      #pkgs.cargo-lambda # is too low of a version to use
    ];
    shellHook = ''
      export EDITOR=emacs
      eval "$(direnv hook bash)"
      echo Welcome to nix-shell for Rust!
    '';
  }
