# Configuration Systems

## How to run

> [!TIP]
> You can use [Nix](https://nixos.org/) for obtaining the dependencies/tools
> or to just compile and run the solvers with one command.
>
> <details>
> <summary>Download all dependencies and tools for this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" develop .#exercise-5
> ```
>
> </details>
> <details>
> <summary>Compile and execute this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" run .#exercise-5
> ```
>
> </details>

You can use cargo to compile and run the code as follows:

```Shell
cargo run -- <path to your dimacs file>
# or compile and then add to path for:
exercise-5 <path to you dimacs file>
```

## Assignment

Almost all software systems are configurable, allowing users to select or
deselect specific features in their software. Such features are usually modeled
with Boolean variables $F = { x_1, x_2 ..., x_n }$, such that a system
configuration is an assignment $c : F \\rightarrow { 0, 1 }$, where for each
feature $x \\in F$:

- $c(x) = 0$ means that $x$ is deselected in $c$.
- $c(x) = 1$ means that $x$ is selected in $c$.

Not all configurations have to lead to an actual software product. For example,
one might disallow an executable that supports both ARM and x86 architectures
at the same time.

The set of _valid configurations_ $V$ is usually provided by a propositional
logic formula $\\varphi$ in conjunctive normal form (CNF), where a configuration
$c$ is valid (i.e., $c \\in V$) if and only if
$\\llbracket \\varphi \\rrbracket(c) = 1$

To compare feature sets, let $F{\\upharpoonright}\_c$ denote the set of features
selected by $c$, namely:
$F{\\upharpoonright}\_c = \\Big{ x \\in F \\text{ }\\Big|\\text{ } c(x) = 1 \\Big}$

For some safety-critical software, industrial certification standards are very
high. On the other hand, configurable software is complex, and activating more
features can cause more bugs to appear. To minimize the certification effort,
it can be useful to use configurations that select as few features as possible.

In this task, you must construct a BDD that represents the set of valid
configurations $V$ and implement the following two techniques for finding small
configurations.

### Sub-question a

A minimum valid configuration is a configuration such that there is no other
configuration that selects fewer features.

More formally, $c\_{min} \\in V$ is minimum if and only if it holds that:

$$
\\forall\_{c'}\\Bigg\[
c' \\in V :
\\Big| F{\\upharpoonright}_{c'} \\Big|
\\geq
\\Big| F{\\upharpoonright}_{c\_{\\min}} \\Big|
\\Bigg\]
$$

Note that minimum valid configurations are not necessarily unique.

### Sub-question b

Some features can be essential, and this should be taken into account when
creating a minimal configuration.

Generalizing the above definition, a valid configuration is minimum for
$S \\subseteq F$ if it is minimum among all configurations that select all
features from $S$.

Formally, $c\_{min} \\in V$ is minimum for $S$ if and only if:
$S{\\upharpoonright}\_{c_min} = S$ and:

$$
\\forall\_{c'}\\Bigg\[
c' \\in V
\\wedge
S{\\upharpoonright}_{c'} = S
:
\\Big| F{\\upharpoonright}_{c'} \\Big|
\\geq
\\Big| F{\\upharpoonright}_{c_{\\min}} \\Big|
\\Bigg\]
$$

### Sub-question c

While finding a single valid configuration can produce a good setup, sometimes
it is useful to reason about all minimum valid configurations. One approach is
to capture these configurations in a BDD.

Describe your implementations, how you find the required minimum valid
configurations, and how you create the BDDs.

Test your implementations on the five configurable systems provided as DIMACS
files in the `data` directory.

For each of the five systems, report the following:

- The total number of BDD nodes used to represent
  $\\llbracket \\varphi \\rrbracket$.
- The total number $\\big| V \\big|$ of valid configurations.
- The size $\\Big| F{\\upharpoonright}_{c_{\\min}} \\Big|$ of a minimum valid
  configuration $c_min$.
- The size $\\Big| F{\\upharpoonright}_{c_{\\min}} \\Big|$ of a minimum valid
  configuration with respect to the set of essential features
  $S = \\Big{ x\_{218}, x\_{311} \\Big}$.
- The number of BDD nodes used to represent the set of all minimum valid
  configurations.
- The number of minimum valid configurations.
