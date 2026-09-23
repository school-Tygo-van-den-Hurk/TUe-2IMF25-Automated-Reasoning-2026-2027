let
  paths = rec {
    scripts = ../scripts;
    run-exercise-interactive = scripts + "/run-exercise-interactive.nu";
  };
in
{
  perSystem = { self', pkgs, ... }: rec {
    apps.default = apps.run-exercise-interactive;
    apps.run-exercise-interactive = {
      inherit (packages.run-exercise-interactive) meta;
      program = pkgs.lib.getExe packages.run-exercise-interactive;
    };

    packages.nushell = pkgs.nushell;
    packages.run-exercise-interactive = pkgs.writeTextFile rec {
      name = "run-exercise-interactive";
      executable = true;
      destination = "/bin/${name}";
      text = ''
        #!${self'.packages.nushell}/bin/nu
        ${builtins.readFile paths.${name}}
      '';
    };
  };
}
