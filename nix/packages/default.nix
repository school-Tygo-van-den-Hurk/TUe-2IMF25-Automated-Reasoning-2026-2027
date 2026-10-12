{
  imports = [
    ./exercise-1.nix
    ./exercise-3.nix
    ./exercise-4.nix
    ./report.nix
  ];

  perSystem = { pkgs, ... }: {
    packages = {
      inherit (pkgs.stdenv) cc;

      inherit (pkgs) texliveFull;
      inherit (pkgs) rust-analyzer;
      inherit (pkgs) rustfmt;
      inherit (pkgs) clippy;
      inherit (pkgs) rustc;
      inherit (pkgs) cargo;
      inherit (pkgs) act;
      inherit (pkgs) git;
      inherit (pkgs) z3;
      inherit (pkgs) gnumake;
      inherit (pkgs) cmake;
      inherit (pkgs) gdb;
      inherit (pkgs) uv;
    };
  };
}
