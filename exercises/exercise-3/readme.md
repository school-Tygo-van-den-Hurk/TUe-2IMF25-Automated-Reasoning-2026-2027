# Food Supply

## How to Run

> [!TIP]
> You can use [Nix](https://nixos.org/) for obtaining the dependencies/tools
> or to just compile and run the solvers with one command.
>
> <details><summary>Download all dependencies and tools for this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" develop .#exercise-3
> ```
>
> </details>
> <details><summary>Compile and execute this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" run .#exercise-3
> ```
>
> </details>

You can use cargo to compile and run the code as follows:

```Shell
cargo run
# or compile and then add to path for:
exercise-3
```

There are 3 subcommands you can use:

- `solve`: Solve the model, and print the results. For a this means find what
  model is UNSAT. Print all SAT models. for `b` this means find a SAT model,
  and print the satisfying assignment.
- `check`: Similar to solve, but doesn't print the final model.
- `print`: Prints the model in SMT2 format.

You can also view a particular sub-question only. For example to check
only question 3a:

```Shell
exercise-3 check a
```

These are the two parameters you can combine. Some more examples:

```Shell
exercise-3 check both
exercise-3 solve a
exercise-3 print b
```

## Assignment

Three non-self-supporting villages $A$, $B$, and $C$ consume one food package
each per time unit. The required food packages are delivered by a truck, having
a capacity of 130 food packages. The truck has to pick up its food packages at
location S containing an unbounded supply. The locations of this supply
location and the villages and the roads are shown in the table below.

<div align="center">
    <table>
    <thead>
        <tr>
        <th><i>p1</i></th>
        <th><i>p2</i></th>
        <th><i>p1 -> p2</i></th>
        <th><i>p1 <- p2</i></th>
        </tr>
    </thead>
    <tbody>
        <tr><td>S</td><td>A</td><td>15</td><td>15</td></tr>
        <tr><td>A</td><td>B</td><td>17</td><td>17</td></tr>
        <tr><td>A</td><td>C</td><td>12</td><td>12</td></tr>
        <tr><td>B</td><td>C</td><td>13</td><td>9</td></tr>
        <tr><td>S</td><td>C</td><td>15</td><td>15</td></tr>
    </tbody>
    </table>
</div>

Here every number indicates the number of time units the truck needs to travel
from one village to another, including loading and delivering. There is often
a lot of traffic going from B to C, hence it takes a bit longer than the
journey from C to B. The villages only have a limited capacity to store food
packages: for A and C this capacity is 90, for B it is 120. Initially, the
truck is in S and is fully loaded, and in each of the three villages there
are 60 food packages.

### Sub-question a

Show that it is impossible to deliver food packages in such a way that each of
the villages consumes one food package per time unit forever. For how long can
the villages be kept alive at best?

### Sub-question b

Show that this is possible if the capacity of the truck is increased to 150
food packages. What is the shortest possible driving schedule for the truck
to sustain the villages indefinitely?

> [!TIP]
> a finite graph contains an infinite path starting in a node $v$ if and only
> if there is a path from $v$ to a node $w$ for which there is a non-empty path
> from $w$ to itself.
