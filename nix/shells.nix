{
  perSystem =
    {
      config,
      self',
      pkgs,
      ...
    }:
    let
      pure = true;

      formatters = builtins.attrValues config.treefmt.build.programs;
      hooks = config.pre-commit.settings.enabledPackages;
      packages = with self'.packages; [
        rust-analyzer # For Rust IDE integration
        texliveFull # Compile latex documents into PDFs
        python3 # an interpreter for Python batteries included
        rustfmt # Format Rust Code
        clippy # Lint Rust Code
        rustc # Compile Rust Code
        cargo # The main part of the rust toolchain
        act # Run your GitHub Actions locally
        git # Distributed version control system
        cc # A C compiler, used by rustc
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
