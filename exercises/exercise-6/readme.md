# Graph exploration

## How to run

> [!TIP]
> You can use [Nix](https://nixos.org/) for obtaining the dependencies/tools
> or to just compile and run the solvers with one command.
>
> <details>
> <summary>Download all dependencies and tools for this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" develop .#exercise-6
> ```
>
> </details>
> <details>
> <summary>Compile and execute this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" run .#exercise-6
> ```
>
> </details>

TODO add later.

## Assignment

One prominent application of binary decision diagrams (BDDs) is in representing
and reasoning about graphs. In this task, we consider directed graphs with
labels on the nodes. More formally, a node-labelled graph is a tuple
$G = (S, T, L)$, where $S$ is a finite set of states, $T \\subseteq S \\times S$
is a transition relation and $L : S \\rightarrow { 0, 1 }$ is a labelling
function. Note that we restrict ourselves to a binary labelling function here.
To denote that $(s, s') \\in T$ , we also write $s \\rightarrow s'$.

The Canvas page of the course provides the file graphs.zip, containing CSV
files representing a few such graphs. One is extracted from
[Twitter][twitter.com] and one from [Pokec][pokec.com] (a Slovakian social
network) and there are two reduced variants of the latter. For each graph, the
[nodes.csv](./data/nodes.csv) file lists the nodes and their labels and the
[edges.csv](./data/edges.csv) file lists one transition $s \\rightarrow s'$ on
each line (and possibly other, irrelevant data). In this task, you have to use
BDDs to represent and explore these graphs.

For each sub-question justify your approach. Report on the number of BDD nodes
for the BDDs used to solve the task and clearly state what these BDDs represent.
As a hint, the graph should be represented by the transition relation and the
labelling function. Note that BDD nodes appearing in two component
representations, e.g., labelling function and transition relation, should only
be counted once. It is recommended to work on the smaller graphs first, since
computation for the larger graphs will take a significant amount of time and
memory.

### Sub-question a

Represent the graph by one or more BDDs choosing a suitable encoding.

### Sub-question b

Determine how many nodes are reachable when starting from the node with id 1.

### Sub-question c

Do the same, but now only count nodes that are reachable via alternatingly $0$
and $1$ labelled nodes. That is count the number of nodes reachable via a path
$s_0 \\rightarrow s_1 \\rightarrow s_2 \\rightarrow s_3 \\rightarrow \\dots$, where
$s_0$ is the node with id $1$ (other nodes may have arbitrary identifiers)
and, for all $i$, $L(s_i) = 0$ iff $i$ is even.

[pokec.com]: https://snap.stanford.edu/data/soc-Pokec.html
[twitter.com]: https://snap.stanford.edu/data/congress-twitter.html
