{
  perSystem =
    {
      config,
      self',
      pkgs,
      lib,
      ...
    }:
    let
      inherit (lib) makeLibraryPath;

      formatters = builtins.attrValues config.treefmt.build.programs;
      hooks = config.pre-commit.settings.enabledPackages;
      packages = with self'.packages; [
        rust-analyzer # For Rust IDE integration
        texliveFull # Compile latex documents into PDFs
        python313 # python code interpreter
        cc.cc.lib # C libraries
        rustfmt # Format Rust Code
        gnumake # Build C++ Code
        git-lfs # commit large files to git.
        clippy # Lint Rust Code
        rustc # Compile Rust Code
        cargo # The main part of the rust toolchain
        cmake # Build C++ Code
        gdb # C++ Debugger
        act # Run your GitHub Actions locally
        git # Distributed version control system
        cc # A C compiler, used by rustc
        z3 # A SAT solver needed for the assignment
        uv # a proper Python package manager
      ];

      libraries = with self'.packages; [
        cc.cc.lib
        zlib
      ];

      buildInputs = packages ++ formatters ++ hooks;

      env = {
        UV_PYTHON_DOWNLOADS = "manual";
        UV_PYTHON_PREFERENCE = "only-system";
        LD_LIBRARY_PATH = makeLibraryPath libraries;
      };

      inherit (config.pre-commit) shellHook;
    in
    rec {
      devShells.default = devShells.development;
      devShells.development = pkgs.mkShell {
        inherit buildInputs;
        inherit shellHook;
        inherit env;
      };
    };
}
