{
  imports = [
    ./exercise-1.nix
    ./report.nix
  ];

  perSystem = { pkgs, ... }: {
    packages = {
      inherit (pkgs) texliveFull;
      inherit (pkgs) rust-analyzer;
      inherit (pkgs.stdenv) cc;
      inherit (pkgs) rustfmt;
      inherit (pkgs) clippy;
      inherit (pkgs) rustc;
      inherit (pkgs) cargo;
      inherit (pkgs) act;
      inherit (pkgs) git;
      inherit (pkgs) z3;

      python3 = pkgs.python3.withPackages (
        pythonPackages: with pythonPackages; [
          z3-solver
          matplotlib
          pytest
          sympy
          numpy
          pandas
          networkx
          ipython
        ]
      );
    };
  };
}
