let
  name = "run-exercise-interactive";
  paths = rec {
    scripts = ../scripts;
    ${name} = scripts + "/${name}.nu";
  };
in
{
  lib,
  ...
}:
let
  inherit (lib) getExe;
in
{
  perSystem = { self', pkgs, ... }: rec {
    apps.default = apps.${name};
    apps.${name} = {
      inherit (packages.${name}) meta;
      program = getExe packages.${name};
    };

    packages.nushell = pkgs.nushell;
    packages.${name} = pkgs.writeTextFile {
      inherit name;
      executable = true;
      destination = "/bin/${name}";
      meta.description = "Run all the exercises interactively.";
      text = ''
        #!${getExe self'.packages.nushell}
        ${builtins.readFile paths.${name}}
      '';
    };
  };
}
