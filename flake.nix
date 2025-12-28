{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake {inherit inputs;} (top: {
      # I can't test support for other systems right now
      systems = ["x86_64-linux"];

      perSystem = {
        lib,
        config,
        pkgs,
        system,
        ...
      }: {
        # Add the fenix overlay when building pkgs in flake-parts
        _module.args.pkgs = import inputs.nixpkgs {
          inherit system;
          overlays = [
            inputs.fenix.overlays.default
          ];
        };

        devShells.default = pkgs.mkShell {
          LD_LIBRARY_PATH = lib.makeLibraryPath [pkgs.openssl];

          packages = with pkgs; [
            pkg-config
            openssl
            watchexec

            (fenix.stable.withComponents [
              "cargo"
              "clippy"
              "rust-src"
              "rustc"
              "rustfmt"
            ])
            rust-analyzer
          ];
        };
      };
    });
}
