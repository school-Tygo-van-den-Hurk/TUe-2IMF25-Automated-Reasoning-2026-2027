# Magic Factory

## How to run

You can use cargo to compile and run the code as follows:

```Shell
cargo run
# or compile and then add to path for:
exercise-1
```

There are 3 subcommands you can use:

- `solve`: Solve the model, and print the results.
- `check`: Checks if the model is SAT or UNSAT.
- `print`: Prints the model in SMT2 format.

> [!WARNING]
> The `print` function is implemented myself and might not be complete,
> or correct.

You can also view a particular sub-question only. For example to check
only question 1a:

```Shell
exercise-1 check a
```

These are the two parameters you can combine. Some more examples:

```Shell
exercise-1 check both
exercise-1 solve a
exercise-1 print b
```

> [!TIP]
> On unix, a nice way to get the results it by sorting the output:
>
> ```Shell
> ; echo question 1a: && exercise-1 solve a | sort;
> ; echo question 1b: && exercise-1 solve b | sort;
> ```
>
> This prevents the output from being randomly sorted and makes it easier
> to read.

## Assignment

Six trucks (with rated speed 80km/h) have to deliver obscure building
blocks to a magic factory. The distance to the factory is 120 km. Every
truck has a max load of 8000 kg and can fit at most 10 building blocks.
In total, the following has to be delivered:

- 6 nuzzles, each of weight 800 kg.
- 12 prittles, each of weight 405 kg.
- 15 skipples, each of weight 500 kg.
- 8 crottles, each of weight 2500 kg.
- A number of dupples, each of weight 600 kg.

If more than two types of goods are present in a truck, it has to drive at
a reduced speed of 60 km/h to lower the risk of magic interactions between
the goods. Skipples are round and elastic, only two out of the six trucks
have facilities for fastening them. Prittles are somewhat valuable: those
pallets need to be distributed over at least five trucks.

### Sub-question a

Investigate what is the maximum number of dupples that can be delivered in
a single run, and show how for that number all goods may be divided over the
six trucks.

### Sub-question b

Do the same, with the extra information that crottles get too cold if there
are less than two dupples in the same truck.
