let
  paths = rec {
    exercises = ../../exercises;
    exercise-4 = exercises + "/exercise-4";
  };
in
{
  perSystem = { pkgs, self', ... }: {
    packages.exercise-4 = pkgs.stdenv.mkDerivation (_finalAttrs: {
      name = "exercise-4";

      src = paths.exercise-4;

      nativeBuildInputs = with self'.packages; [ cmake ];
      buildInputs = with self'.packages; [ z3 ];
    });
  };
}
