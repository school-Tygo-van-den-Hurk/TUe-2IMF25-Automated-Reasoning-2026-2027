# Tour du la City

## How to run

> [!TIP]
> You can use [Nix](https://nixos.org/) for obtaining the dependencies/tools
> or to just compile and run the solvers with one command.
>
> <details>
> <summary>Download all dependencies and tools for this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" develop .#exercise-7
> ```
>
> </details>
> <details>
> <summary>Compile and execute this exercise.</summary>
>
> ```Shell
> nix --experimental-features "nix-command flakes" run .#exercise-7
> ```
>
> </details>

TODO: fill in later when the exercise is completed...

## Assignment

The $ABC 123 XYZ$ city has been asked to be a location in new formula 1 season.
The major has tasked you to find a optimal route throughout the city. They have
assembled the following requirements for the route:

- All cars must use existing roads. The city does not wish to build more roads,
  nor can they destroy existing buildings.
- You must visit $m$ out of $n$ special locations throughout the city.
- The length of the circuit must be between $k$ and $l$.
- To prevent injuries and deaths the circuit may never cross itself.
- It must have between $a$ and $b$ corners.
- As they need to race 50 laps around it, the start location and end location
  need to be the same place. (The circuit needs to be a full circle.)
- Because of noise pollution you are tasked to minimize the amount of internal
  area of the circuit to a minimum given the other requirements.
