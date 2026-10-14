# Configuration Systems

## How to run

> [!TIP]
> You can use [Nix](https://nixos.org/) for obtaining the dependencies/tools
> or to just compile and run the solvers with one command.
>
> <details><summary>Download all dependencies and tools for this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" develop .#exercise-1
> ```
>
> </details>
> <details><summary>Compile and execute this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" run .#exercise-1
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

Almost all software systems are configurable, allowing users to select or deselect specific features in their software. Such features are usually modeled with Boolean variables F={x1,…,xn}, such that a system configuration is an assignment c:F→{0,1}, where for each feature x∈F:

c(x)=0 means that x is deselected in c.
c(x)=1 means that x is selected in c.

Not all configurations have to lead to an actual software product. For example, one might disallow an executable that supports both ARM and x86 architectures at the same time.

The set of valid configurations V is usually provided by a propositional logic formula φ in conjunctive normal form (CNF), where a configuration c is valid (i.e., c ∈ V) if and only if:

⟦φ⟧(c)=1

To compare feature sets, let F↾c denote the set of features selected by c, namely:

F↾c={ x ∈ F ∣ c(x) = 1}

For some safety-critical software, industrial certification standards are very high. On the other hand, configurable software is complex, and activating more features can cause more bugs to appear. To minimize the certification effort, it can be useful to use configurations that select as few features as possible.

In this task, you must construct a BDD that represents the set of valid configurations V and implement the following two techniques for finding small configurations.

(a) Minimum valid configuration

A minimum valid configuration is a configuration such that there is no other configuration that selects fewer features.

More formally, cmin ∈ V is minimum if and only if, for all c′ ∈ V, it holds that:

∣F↾c′∣≥∣F↾cmin∣

Note that minimum valid configurations are not necessarily unique.

(b) Minimum valid configuration with essential features

Some features can be essential, and this should be taken into account when creating a minimal configuration.

Generalizing the above definition, a valid configuration is minimum for S ⊆ F if it is minimum among all configurations that select all features from S.

Formally, cmin ∈ V is minimum for S if and only if:

S↾cmin=S

and, for all c′ such that S↾c′=S, it holds that:

∣F↾c′∣≥∣F↾cmin∣
(c) Representing all minimum valid configurations

While finding a single valid configuration can produce a good setup, sometimes it is useful to reason about all minimum valid configurations. One approach is to capture these configurations in a BDD.

Describe your implementations, how you find the required minimum valid configurations, and how you create the BDDs.

Test your implementations on the five configurable systems provided as DIMACS files in conf-dimacs.zip from the course's Canvas folder. See the appendix for further descriptions of the input format.

For each of the five systems, report the following:

- The total number of BDD nodes used to represent ⟦φ⟧.
- The total number ∣V∣ of valid configurations.
- The size ∣F↾cmin∣ of a minimum valid configuration cmin.
- The size ∣F↾cmin∣ of a minimum valid configuration with respect to the set of essential features S={x218,x311}.
- The number of BDD nodes used to represent the set of all minimum valid configurations.
- The number of minimum valid configurations.
