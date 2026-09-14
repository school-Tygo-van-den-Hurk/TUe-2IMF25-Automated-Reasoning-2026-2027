{
  perSystem = { pkgs, ... }: rec {
    devShells.default = devShells.development;
    devShells.development = pkgs.mkShell {
      buildInputs = with pkgs; [
        rust-analyzer
        stdenv.cc
        rustfmt
        clippy
        rustc
        cargo
        git
        act
        z3
      ];
    };
  };
}
