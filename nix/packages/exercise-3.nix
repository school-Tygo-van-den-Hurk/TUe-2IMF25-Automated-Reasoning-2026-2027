{
  lib,
  ...
}:
let
  meta = import ./meta.lib.nix lib;

  paths = rec {
    inherit (import ./paths.lib.nix) exercise-3;
    cargoToml = exercise-3 + "/Cargo.toml";
    cargoLock = exercise-3 + "/Cargo.lock";
  };

  project = builtins.fromTOML (builtins.readFile paths.cargoToml);
in
{
  perSystem = { pkgs, self', ... }: {
    packages.exercise-3 = pkgs.rustPlatform.buildRustPackage (_finalAttrs: {
      inherit (project.package) name;
      inherit (project.package) version;

      buildInputs = with self'.packages; [
        z3
      ];

      src = paths.exercise-3;

      cargoLock.lockFile = paths.cargoLock;

      doInstallCheck = true;
      nativeInstallCheckInputs = with self'.packages; [
        rustfmt
        clippy
      ];

      meta = meta // {
        inherit (project.package) description;
        mainProgram = project.package.name;
      };
    });
  };
}
