{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";
    flake-compat.url = "github:edolstra/flake-compat";
    treefmt-nix = {
      url = "github:numtide/treefmt-nix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{
      nixpkgs,
      flake-parts,
      fenix,
      ...
    }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = nixpkgs.lib.systems.flakeExposed;

      imports = [
        inputs.treefmt-nix.flakeModule
      ];

      perSystem =
        {
          pkgs,
          system,
          ...
        }:
        let
          rust-toolchain = fenix.packages.${system}.complete.toolchain;

          # On liste toutes les bibliothèques nécessaires à Bevy pour le runtime
          runtimeLibs = with pkgs; [
            udev
            alsa-lib
            vulkan-loader
            libxkbcommon
            wayland
            libX11
            libXcursor
            libXi
            libXrandr
          ];
        in
        {
          treefmt = {
            projectRootFile = "flake.lock";
            programs.nixfmt.enable = true;
          };

          devShells.default = pkgs.mkShell {
            nativeBuildInputs = [
              rust-toolchain
              pkgs.pkg-config
              pkgs.cargo-nextest
            ];

            buildInputs = runtimeLibs;

            shellHook = ''
              export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath runtimeLibs}"
            '';
          };
        };
    };
}
