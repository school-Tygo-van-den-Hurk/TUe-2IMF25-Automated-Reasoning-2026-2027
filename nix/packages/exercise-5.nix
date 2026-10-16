{
  lib,
  ...
}:
let
  meta = import ./meta.lib.nix lib;

  paths = rec {
    inherit (import ./paths.lib.nix) exercise-5;
    cargoToml = exercise-5 + "/Cargo.toml";
    cargoLock = exercise-5 + "/Cargo.lock";
  };

  project = builtins.fromTOML (builtins.readFile paths.cargoToml);
in
{
  perSystem = { pkgs, self', ... }: {
    packages."exercise-5" = pkgs.rustPlatform.buildRustPackage {
      inherit (project.package) name;
      inherit (project.package) version;

      src = paths.exercise-5;

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
    };
  };
}
