let
  paths = rec {
    exercises = ../../exercises;
    exercise-4 = exercises + "/exercise-4";
  };
in
{
  perSystem = { pkgs, self', ... }: {
    packages.exercise-4 = pkgs.stdenv.mkDerivation (finalAttrs: {
      name = "exercise-4";

      src = paths.exercise-4;

      nativeBuildInputs = with self'.packages; [ cmake ];
      buildInputs = with self'.packages; [ z3 ];

      meta = with pkgs.lib; {
        homepage = "https://github.com/school-Tygo-van-den-Hurk/TUe-2IMF25-Automated-Reasoning-2026-2027";
        maintainers = with maintainers; [ Tygo-van-den-Hurk ];
        mainProgram = finalAttrs.name;
      };
    });
  };
}
