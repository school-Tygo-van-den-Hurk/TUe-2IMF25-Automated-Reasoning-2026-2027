{
  perSystem =
    {
      config,
      pkgs,
      ...
    }:
    let
      pure = true;

      formatters = builtins.attrValues config.treefmt.build.programs;
      hooks = config.pre-commit.settings.enabledPackages;
      packages = with pkgs; [
        (python3.withPackages (pythonPackages: with pythonPackages; [
          z3-solver matplotlib pytest sympy numpy pandas networkx ipython
        ])) # an interpreter for Python batteries included
        act # Run your GitHub Actions locally
        git # Distributed version control system
        stdenv.cc # A C compiler, used by rustc
        rust-analyzer # For Rust IDE integration
        rustfmt # Format Rust Code
        clippy # Lint Rust Code
        rustc # Compile Rust Code
        cargo # The main part of the rust toolchain
        z3 # A SAT solver needed for the assignment
      ];

      buildInputs = packages ++ formatters ++ hooks;

      shellHook = ''
        ${config.pre-commit.shellHook}
        if [ -f .env ]; then
          source .env
        fi
      '';
    in
    rec {
      devShells.default = devShells.development;
      devShells.development = pkgs.mkShell {
        inherit buildInputs;
        inherit shellHook;
        inherit pure;
      };
    };
}
