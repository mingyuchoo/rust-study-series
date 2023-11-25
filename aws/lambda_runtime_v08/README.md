# README
## References

- <https://crates.io/crates/lambda_runtime>

## Prerequsites

### Install `cargo-lambda`

- <https://www.cargo-lambda.info/guide/installation.html>


If you are using NixOS, please follow this.

add `cargo-lambda` to `/etc/nixos/configuration.nix`

```nix
{ config, pkgs, ... }:
{
    users.users.<username> = {
        packages = with pkgs; [
            cargo-lambda
        ]
    }
}
```


If you are using Nix, please follow this.

```bash
nix-env -iA nixpkgs.cargo-lambda
```

