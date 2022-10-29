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

      nightly-rustfmt = pkgs.rust-bin.selectLatestNightlyWith
          (toolchain: toolchain.minimal.override { extensions = [ "rustfmt" ]; });

      rust-toolchain =
        (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml).override {
          extensions = [ "rust-src" "clippy"];
        };

      format-pkgs = with pkgs; [
        nixpkgs-fmt
      ];

      cargo-installs = with pkgs; [
          cargo-expand
          cargo-sort
          cargo-udeps
          cargo-watch
          evcxr
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
    }
  );
}
