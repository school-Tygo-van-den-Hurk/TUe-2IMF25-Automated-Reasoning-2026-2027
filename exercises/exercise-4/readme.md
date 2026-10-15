# Program Verification

## How to run

> [!TIP]
> You can use [Nix](https://nixos.org/) for obtaining the dependencies/tools
> or to just compile and run the solvers with one command.
>
> <details>
> <summary>Download all dependencies and tools for this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" develop .#exercise-4
> ```
>
> </details>
> <details>
> <summary>Compile and execute this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" run .#exercise-4
> ```
>
> </details>

You can use `cmake` to compile the code as follows:

```Shell
cmake -B build
cmake --build build
```

And run it via:

```Shell
./build/exercise-4
```

The assignment that gets printed is quite verbose, to show a more concise
version you can pipe cat in to this command:

```Shell
tr '\n' ' ' \
| sed -E 's/\(define-fun /\n/g' \
| sed -E '/^$/d; s/^([^ ]+) \(\) [^ ]+ +(.*)\) *$/\1 = \2/' \
| sort -V
```

## Assignment

Consider the following program written in C. Here, `__nondet__()` is a function
that non-deterministically returns an arbitrary Boolean value.

```c
int main(void) {
    int a = 1;
    int b = 1;

    while (a < 180) {
        if (__nondet__()) {
            b = b + 3;
            a = a + 2 * b;
        } else {
            b = b + a;
            a = a + 5;
        }
    }

    if (b == 190 + k) {
        assert(false);
    }
}
```

Show for each value of $k \\in \\big{ 0, 1, ..., 10 \\big}$ whether the program
is safe or can crash (i.e. reach the assert statement). If a crash is possible,
provide one complete run leading to that crash.
