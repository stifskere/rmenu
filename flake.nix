{
  description = "Flake configuration file for translatable development.";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
    fenix.url = "github:nix-community/fenix";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils, fenix, ... }@inputs:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
        crane = inputs.crane.mkLib pkgs;

        rmenuPkg = pkgs.rustPlatform.buildRustPackage {
          pname = "rmenu";
          version = "0.1.0";

          src = ./.;
          cargoLock = { lockFile = ./Cargo.lock; };

          buildInputs = with pkgs; [ SDL2 SDL2_gfx SDL2_ttf ];

          meta = with pkgs.lib; {
            description =
              "rmenu is a rewrite of dmenu originally written in C, but with completions from major shells and icons inferred from the environment.";
            license = with licenses; [ mit apache2 ];
            platforms = platforms.unix;
          };
        };

        toolchain = with fenix.packages.${system};
          combine [
            stable.rustc
            stable.rust-src
            stable.cargo
            complete.rustfmt
            stable.clippy
            stable.rust-analyzer
          ];

        # Override the toolchain in crane
        craneLib = crane.overrideToolchain toolchain;
      in {
        packages = { default = rmenuPkg; };

        apps = {
          rmenu = {
            name = "rmenu";
            type = "app";
            program = "${rmenuPkg}/bin/rmenu";
          };
        };

        devShells.default = craneLib.devShell {
          packages = with pkgs; [ toolchain SDL2 SDL2_gfx SDL2_ttf ];

          env = { LAZYVIM_RUST_DIAGNOSTICS = "bacon-ls"; };
        };
      });
}
