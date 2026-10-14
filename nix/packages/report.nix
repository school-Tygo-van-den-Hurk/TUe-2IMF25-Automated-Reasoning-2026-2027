{
  lib,
  ...
}:
let
  meta = import ./meta.lib.nix lib;

  paths = rec {
    inherit (import ./paths.lib.nix) root;
    inherit (import ./paths.lib.nix) share;
    inherit (import ./paths.lib.nix) docs;
    assignment = share + "/assignment.pdf";
  };
in
{
  perSystem = { pkgs, self', ... }: {
    packages.report = pkgs.stdenv.mkDerivation rec {
      name = "2imf25-automated-reasoning-report";

      src = paths.docs;
      inherit (paths) assignment;

      buildInputs = with self'.packages; [
        texliveFull
      ];

      buildPhase = ''
        runHook preBuild

        LATEX_OUT="$PWD/latex.out"
        mkdir --parents -- "$LATEX_OUT"
        echo "Using: $(pdflatex -version)"

        # First go over the document and create the outlines:
        pdflatex --interaction=nonstopmode --file-line-error \
          --output-directory="$LATEX_OUT" "$PWD/main.tex" \
          --draftmode /dev/null 1> /dev/null 2> /dev/null || true


        # now use those outlines to create the final document:
        pdflatex --interaction=nonstopmode --file-line-error \
          --output-directory="$LATEX_OUT" "$PWD/main.tex" \
          --halt-on-error || true

        [ -e "$LATEX_OUT/main.pdf" ] || {
          echo no output file: "$LATEX_OUT/main.pdf"
          exit 1
        }

        runHook postBuild
      '';

      installPhase = ''
        runHook preInstall

        mkdir --parents -- "$out/share"
        cp "$LATEX_OUT/main.pdf" "$out/share/report.pdf"
        cp "$assignment" "$out/share/assignment.pdf"

        runHook postInstall
      '';

      inherit meta;
    };
  };
}
