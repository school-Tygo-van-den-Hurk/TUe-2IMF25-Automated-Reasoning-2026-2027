let
  name = "exercise-4";
in
{
  lib,
  ...
}:
let
  meta = import ./meta.lib.nix lib;

  paths = { inherit (import ./paths.lib.nix) exercise-4; };
in
{
  perSystem = { pkgs, self', ... }: {
    packages.${name} = pkgs.stdenv.mkDerivation (finalAttrs: {
      inherit name;

      src = paths.${name};

      nativeBuildInputs = with self'.packages; [ cmake ];
      buildInputs = with self'.packages; [ z3 ];

      meta = meta // {
        mainProgram = finalAttrs.name;
      };
    });
  };
}
