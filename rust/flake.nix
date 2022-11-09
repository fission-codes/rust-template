{
  description = "{{project-name}}";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
  };

  outputs =
    { self
    , nixpkgs
    , flake-utils
    , rust-overlay
    } @ inputs:
    flake-utils.lib.eachDefaultSystem (system:
    let
      overlays = [ (import rust-overlay) ];
      pkgs = import nixpkgs { inherit system overlays; };

      rust-toolchain =
        (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = [ "cargo" "clippy" "rustfmt" "rust-src" "rust-std" ];
        };

      nightly-rustfmt = pkgs.rust-bin.nightly.latest.rustfmt;

      format-pkgs = with pkgs; [
        nixpkgs-fmt
      ];

      cargo-installs = with pkgs; [
        cargo-expand
        cargo-sort
        cargo-udeps
        cargo-watch
      ];
    in
    rec
    {
      devShells.default = pkgs.mkShell {
        name = "{{project-name}}";
        nativeBuildInputs = with pkgs; [
          # The ordering of these two items is important. For nightly rustfmt to be used instead of
          # the rustfmt provided by `rust-toolchain`, it must appear first in the list. This is
          # because native build inputs are added to $PATH in the order they're listed here.
          nightly-rustfmt
          rust-toolchain
          pre-commit
          direnv
        ] ++ format-pkgs ++ cargo-installs;

      shellHook = ''
        [ -e .git/hooks/pre-commit ] || pre-commit install --install-hooks
      '';
      };

      checks = {
        format = pkgs.runCommand
          "check-nix-format"
          { buildInputs = format-pkgs; }
          ''
            ${pkgs.nixpkgs-fmt}/bin/nixpkgs-fmt --check ${./.}
            touch $out
          '';
      };
    }
  );
}
