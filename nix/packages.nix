{
  perSystem = { pkgs, ... }: rec {
    packages = {
      inherit (pkgs) rust-analyzer;
      inherit (pkgs.stdenv) cc;
      inherit (pkgs) rustfmt;
      inherit (pkgs) clippy;
      inherit (pkgs) rustc;
      inherit (pkgs) cargo;
      inherit (pkgs) z3;
    };
  };
}
