{
  inputs = {
    nixpkgs = {
      url = "github:nixos/nixpkgs/nixos-unstable";
    };
    flake-parts.url = "github:hercules-ci/flake-parts";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };
  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          runtimeDeps = with pkgs; [
          ];
          buildDeps = with pkgs; [
            clang
            lld
            lldb
            pkg-config
            rustPlatform.bindgenHook
          ];
          devDeps = with pkgs; [
            cargo-msrv
            cargo-nextest
            cargo-watch
            clang
            just
            gdb
            lld
            lldb
            nushell
            surrealdb
            panamax
          ];

          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          msrv = cargoToml.package.rust-version;

          rustPackage = features:
            (pkgs.makeRustPlatform {
              cargo = pkgs.rust-bin.stable.latest.minimal;
              rustc = pkgs.rust-bin.stable.latest.minimal;
            }).buildRustPackage {
              inherit (cargoToml.package) name version;
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              buildFeatures = features;
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps;
              # Uncomment if your cargo tests require networking or otherwise
              # don't play nicely with the nix build sandbox:
              # doCheck = false;
            };

          mkDevShell = rustc:
            pkgs.mkShell {
              shellHook = ''
                export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}
              '';
              LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib";
              buildInputs = runtimeDeps;
              nativeBuildInputs = buildDeps ++ devDeps ++ [ rustc ];
            };
        in
        {
          _module.args.pkgs = import inputs.nixpkgs
            {
              inherit system;
              overlays = [ (import inputs.rust-overlay) ];
              config.allowUnfreePredicate = pkg: builtins.elem (lib.getName pkg) [
                "surrealdb"
              ];
            };

          packages.default = self'.packages.base;
          devShells.default = self'.devShells.stable;

          packages.example = (rustPackage "foobar");
          packages.example-base = (rustPackage "");
          packages.base = (rustPackage "");
          packages.bunyan = (rustPackage "bunyan");
          packages.tokio-console = (rustPackage "tokio-console");

          devShells.nightly = (mkDevShell (pkgs.rust-bin.selectLatestNightlyWith
            (toolchain: toolchain.default.override {
              extensions = [ "rust-analyzer" ];
            })));
          devShells.stable = (mkDevShell (pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-analyzer" ];
          }));
          devShells.msrv = (mkDevShell (pkgs.rust-bin.stable.${msrv}.default.override {
            extensions = [ "rust-analyzer" ];
          }));
        };
    };
}
