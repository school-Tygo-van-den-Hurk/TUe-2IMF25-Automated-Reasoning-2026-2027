let
  name = "exercise-6";
in
{
  inputs,
  lib,
  ...
}:
let
  meta = import ./meta.lib.nix lib;

  paths = rec {
    inherit (import ./paths.lib.nix) exercise-6;
    toml = exercise-6 + "/pyproject.toml";
  };

  pyproject = builtins.fromTOML (builtins.readFile paths.toml);

  inherit (inputs) pyproject-build-systems;
  inherit (inputs) pyproject-nix;
  inherit (inputs) uv2nix;
  
  inherit (uv2nix.lib.workspace) loadWorkspace;

  workspace = loadWorkspace {
    workspaceRoot = paths.exercise-6;
  };
in
{
  perSystem = 
    {
      self',
      pkgs,
      ...
    }:
    let
      inherit (pyproject-nix.build) packages;
      pythonBase = pkgs.callPackage packages {
        python = self'.packages.python313;
      };

      overlay = workspace.mkPyprojectOverlay {
        sourcePreference = "wheel";
      };

      inherit (pyproject-build-systems.overlays) wheel;
      pythonSet = pythonBase.overrideScope (
        lib.composeManyExtensions [
          wheel
          overlay
        ]
      );

      package = pythonSet.mkVirtualEnv name {
        ${name} = [ ];
      };
    in
    {
      packages.${name} = package // {
        meta = meta // {
          inherit (pyproject.project) description;
          mainProgram = name;
        };
      };
    };
}
