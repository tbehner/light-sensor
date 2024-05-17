{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = [
    pkgs.rustup
    pkgs.cargo
    pkgs.clippy
    pkgs.rust-analyzer
    pkgs.rustfmt
    pkgs.cargo-generate
    pkgs.ravedude
    # Add any other dependencies specific to your project here
  ];

  shellHook = ''
    export CARGO_HOME=$PWD/.cargo
    export RUSTUP_HOME=$PWD/.rustup
  '';
}
