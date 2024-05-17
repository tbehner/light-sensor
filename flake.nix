{
  description = "Development shell for a Rust project";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }: 
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        overrides = (builtins.fromTOML (builtins.readFile ./rust-toolchain.toml));
        libPath = with pkgs; lib.makeLibraryPath [
          # load external libraries that you need in your rust project here
        ];
        avrlibc = pkgs.pkgsCross.avr.libcCross;

        avr_incflags = [
          "-isystem ${avrlibc}/avr/include"
          "-B${avrlibc}/avr/lib/avr5"
          "-L${avrlibc}/avr/lib/avr5"
          "-B${avrlibc}/avr/lib/avr35"
          "-L${avrlibc}/avr/lib/avr35"
          "-B${avrlibc}/avr/lib/avr51"
          "-L${avrlibc}/avr/lib/avr51"
        ];
      in
        with pkgs;
        {
          devShells.default = mkShell {
            buildInputs = [
              pkgs.rust-analyzer
              pkgs.cargo-generate
              pkgs.ravedude
              pkgs.rustup
              pkgs.clang
              # Replace llvmPackages with llvmPackages_X, where X is the latest LLVM version (at the time of writing, 16)
              pkgs.llvmPackages.bintools
              rust-bin.stable.latest.default
              pkgs.pkgsCross.avr.buildPackages.binutils
              pkgs.pkgsCross.avr.buildPackages.gcc8
              pkgs.avrdude
            ];
          };
          RUSTC_VERSION = overrides.toolchain.channel;
          # https://github.com/rust-lang/rust-bindgen#environment-variables
          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
          shellHook = ''
            export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
          '';
          # Add precompiled library to rustc search path
          RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
            # add libraries here (e.g. pkgs.libvmi)
          ]);
          LD_LIBRARY_PATH = libPath;
          # Add glibc, clang, glib, and other headers to bindgen search path
          BINDGEN_EXTRA_CLANG_ARGS =
            # Includes normal include path
            (builtins.map (a: ''-I"${a}/include"'') [
              # add dev libraries here (e.g. pkgs.libvmi.dev)
              pkgs.glibc.dev
            ])
            # Includes with special directory paths
            ++ [
              ''-I"${pkgs.llvmPackages_latest.libclang.lib}/lib/clang/${pkgs.llvmPackages_latest.libclang.version}/include"''
              ''-I"${pkgs.glib.dev}/include/glib-2.0"''
              ''-I${pkgs.glib.out}/lib/glib-2.0/include/''
            ];
        }
    );

}
