{
  perSystem = { pkgs, ... }: rec {
    devShells.default = devShells.development;
    devShells.development = pkgs.mkShell {
      buildInputs = with pkgs; [
        (python3.withPackages (pythonPackages: with pythonPackages; [
          z3-solver matplotlib pytest sympy numpy pandas networkx ipython
        ]))
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
