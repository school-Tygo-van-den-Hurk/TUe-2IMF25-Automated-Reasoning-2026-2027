#!/usr/bin/env nu

let esc = "\u{1b}"
let spinner = ["⠋" "⠙" "⠹" "⠸" "⠼" "⠴" "⠦" "⠧" "⠇" "⠏"]

let nix_flake_url = try {
    git rev-parse --show-toplevel | str trim
} catch {
    "github:school-Tygo-van-den-Hurk/TUe-2IMF25-Automated-Reasoning-2026-2027"
}

try {
    # print -n $"($esc)[?1049h"
    print -n $"($esc)[2J($esc)[H"
    loop {
        print "which exercise would you like to run?"
        print '(enter a number from 1 till 7 or "quit")'
        let exercise = input "> "
        print ""

        if $exercise == "q" or $exercise == "quit" {
            break
        }

        let n = try {
            $exercise | into int
        } catch {
            print $"(ansi red)Error:(ansi reset) please enter a number between 1 and 7."
            continue
        }

        if $n < 1 or $n > 7 {
            print $"(ansi red)Error:(ansi reset) please enter a number between 1 and 7."
            continue
        }

        let target = $"($nix_flake_url)#exercise-($n)"
        let result_file = $"/tmp/exercise-build-($n).result"

        job spawn {
            let result = (^nix build $target | complete)

            {
                exit_code: $result.exit_code
                stderr: $result.stderr
            } | to json | save -f $result_file
        }

        mut i = 0

        while not ($result_file | path exists) {
            print -n $"\r($esc)[2K(ansi cyan)($spinner | get $i)(ansi reset) Building exercise-($n)..."
            $i = ($i + 1) mod ($spinner | length)
            sleep 100ms
        }

        let result = open $result_file | from json
        rm $result_file

        if $result.exit_code != 0 {
            print -n $"\r($esc)[2K"
            print $result.stderr
            print $"(ansi red)Build failed.(ansi reset)"
            continue
        }

        print -n $"\r($esc)[2K(ansi green)Build complete!(ansi reset)\n"
        ^nix run $target
    }
} catch {
    print $"(ansi red)Error:(ansi reset) unknown."
} finally {
    # print -n $"($esc)[?1049l"
}
