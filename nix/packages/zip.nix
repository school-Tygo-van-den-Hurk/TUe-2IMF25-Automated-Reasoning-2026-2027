let
  name = "2imf25.zip";
in
{
  inputs,
  lib,
  ...
}:
let
  inherit (inputs) self;
  meta = (import ./meta.lib.nix lib) // {
    description = "Zip the whole repository up into a zip file.";
  };
in
{
  perSystem = { self', pkgs, ... }: {
    packages."zip" = pkgs."zip";

    apps."zip" = rec {
      type = "app";
      inherit meta;
      program = pkgs.writeShellApplication {
        inherit name;
        inherit meta;
        runtimeInputs = with self'.packages; [ zip ];
        text = /* Shell */ ''
          file="$(mktemp)"
          trap "rm -f '$tmpfile'" EXIT
          zip -r "$file" -9 '${self}'
          mv -- "$file" "$PWD/result"
        '';
      };
    };

    packages.${name} =
      pkgs.runCommand name
        {
          nativeBuildInputs = with self'.packages; [ zip ];
          inherit meta;
        }
        /* Shell */ ''
          zip -r "$out" -9 '${self}'
        '';
  };
}
