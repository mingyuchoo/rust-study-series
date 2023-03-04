{
  description = "A basic devShell using flake-utils for Rust";

  inputs.flake-utils.url = "github:numtide/flake-utils";

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system};
      in
        rec {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              cargo-watch
              clippy
              direnv
              gcc
              rustc
              rustfmt
              rustup
            ];
            shellHook = ''
              export EDITOR=emacs
              eval "$(direnv hook bash)"
              echo "Welcome to nix flake for Rust"
            '';
          };
        }
    );
}
