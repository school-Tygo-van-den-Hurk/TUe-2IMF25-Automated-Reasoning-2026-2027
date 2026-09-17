let
  paths = rec {
    root = ../..;
    share = root + "/share";
    assignment = share + "/assignment.pdf";
    src = root + "/docs";
  };
in
{
  perSystem = { pkgs, self', ... }: {
    packages.report = pkgs.stdenv.mkDerivation rec {
      name = "2imf25-automated-reasoning-report";

      inherit (paths) src;
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
    };
  };
}
