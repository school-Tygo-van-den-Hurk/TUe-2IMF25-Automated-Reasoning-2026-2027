let
  paths = rec {
    exercises = ../../exercises;
    exercise-1 = exercises + "/exercise-1";
    cargoToml = exercise-1 + "/Cargo.toml";
    cargoLock = exercise-1 + "/Cargo.lock";
  };

  cargoToml = builtins.fromTOML (builtins.readFile paths.cargoToml);
in
{
  perSystem = { pkgs, self', ... }: {
    packages.exercise-1 = pkgs.rustPlatform.buildRustPackage (_finalAttrs: {
      inherit (cargoToml.package) name;
      inherit (cargoToml.package) version;

      buildInputs = with self'.packages; [
        z3
      ];

      src = paths.exercise-1;

      cargoLock.lockFile = paths.cargoLock;

      doInstallCheck = true;
      nativeInstallCheckInputs = with self'.packages; [
        rustfmt
        clippy
      ];

      checkPhase = /* SH */ ''
        runHook preCheck
        echo  cargo clippy -- --deny warnings
              cargo clippy -- --deny warnings
        echo  cargo test --all-features
              cargo test --all-features
        echo  cargo fmt -- --check
              cargo fmt -- --check
        runHook postCheck
      '';

      meta = with pkgs.lib; rec {
        inherit (cargoToml.package) description;
        homepage = "https://github.com/school-Tygo-van-den-Hurk/TUe-2IMF25-Automated-Reasoning-2026-2027";
        maintainers = with maintainers; [ Tygo-van-den-Hurk ];
        mainProgram = cargoToml.package.name;
      };
    });
  };
}
