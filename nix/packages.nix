let
  paths = rec {
    exercises = ../exercises;
    exercise-1 = exercises + "/exercise-1";
  };
in
{
  perSystem = { pkgs, ... }: {
    packages = {
      inherit (pkgs) rust-analyzer;
      inherit (pkgs.stdenv) cc;
      inherit (pkgs) rustfmt;
      inherit (pkgs) clippy;
      inherit (pkgs) rustc;
      inherit (pkgs) cargo;
      inherit (pkgs) z3;

      exercise-1 =
        let
          cargoToml = builtins.fromTOML (builtins.readFile (paths.exercise-1 + "/Cargo.toml"));
        in
        pkgs.rustPlatform.buildRustPackage (finalAttrs: {
          inherit (cargoToml.package) name;
          inherit (cargoToml.package) version;

          buildInputs = [
            pkgs.z3
          ];

          src = paths.exercise-1;

          cargoLock.lockFile = paths.exercise-1 + "/Cargo.lock";

          doInstallCheck = true;
          nativeInstallCheckInputs = with pkgs; [
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
            changelog = "${homepage}/releases/tag/v${finalAttrs.version}";
            license = with licenses; [ mit ];
            maintainers = with maintainers; [ Tygo-van-den-Hurk ];
            mainProgram = cargoToml.package.name;
          };
        });
    };
  };
}
