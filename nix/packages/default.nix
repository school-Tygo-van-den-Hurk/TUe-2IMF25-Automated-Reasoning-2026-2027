{
  imports = [
    ./exercise-1.nix
    ./exercise-3.nix
    ./exercise-4.nix
    ./exercise-5.nix
    ./exercise-6.nix
    ./report.nix
    ./zip.nix
  ];

  perSystem = { pkgs, ... }: {
    packages = {
      inherit (pkgs.stdenv) cc;

      inherit (pkgs) rust-analyzer;
      inherit (pkgs) texliveFull;
      inherit (pkgs) makeWrapper;
      inherit (pkgs) python313;
      inherit (pkgs) rustfmt;
      inherit (pkgs) gnumake;
      inherit (pkgs) git-lfs;
      inherit (pkgs) clippy;
      inherit (pkgs) rustc;
      inherit (pkgs) cargo;
      inherit (pkgs) cmake;
      inherit (pkgs) zlib;
      inherit (pkgs) gdb;
      inherit (pkgs) act;
      inherit (pkgs) git;
      inherit (pkgs) z3;
      inherit (pkgs) uv;
    };
  };
}
