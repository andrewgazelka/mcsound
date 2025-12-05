{
  description = "Play Minecraft sounds from the CLI";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
    ...
  }: let
    systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];
    forAllSystems = f: nixpkgs.lib.genAttrs systems (system: f system);
  in {
    packages = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      inherit (pkgs) lib;

      craneLib = crane.mkLib pkgs;
      src = craneLib.cleanCargoSource ./.;

      # Audio dependencies per platform
      # Note: Darwin frameworks are provided automatically by the new stdenv
      audioDeps =
        if pkgs.stdenv.isLinux
        then [pkgs.alsa-lib]
        else [];

      commonArgs = {
        inherit src;
        strictDeps = true;

        nativeBuildInputs = lib.optionals pkgs.stdenv.isLinux [
          pkgs.pkg-config
        ];

        buildInputs =
          audioDeps
          ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;

      mcsound = craneLib.buildPackage (commonArgs // {inherit cargoArtifacts;});
    in {
      inherit mcsound;
      default = mcsound;
    });

    checks = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      inherit (pkgs) lib;

      craneLib = crane.mkLib pkgs;
      src = craneLib.cleanCargoSource ./.;

      audioDeps =
        if pkgs.stdenv.isLinux
        then [pkgs.alsa-lib]
        else [];

      commonArgs = {
        inherit src;
        strictDeps = true;

        nativeBuildInputs = lib.optionals pkgs.stdenv.isLinux [
          pkgs.pkg-config
        ];

        buildInputs =
          audioDeps
          ++ lib.optionals pkgs.stdenv.isDarwin [
            pkgs.libiconv
          ];
      };

      cargoArtifacts = craneLib.buildDepsOnly commonArgs;
    in {
      mcsound = self.packages.${system}.mcsound;

      mcsound-clippy = craneLib.cargoClippy (commonArgs // {
        inherit cargoArtifacts;
        cargoClippyExtraArgs = "--all-targets -- --deny warnings";
      });

      mcsound-fmt = craneLib.cargoFmt {inherit src;};
    });

    devShells = forAllSystems (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      craneLib = crane.mkLib pkgs;
    in {
      default = craneLib.devShell {
        checks = self.checks.${system};
        packages = [pkgs.rust-analyzer];
      };
    });
  };
}
