#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""
One prominent application of binary decision diagrams (BDDs) is in representing
and reasoning about graphs. In this task, we consider directed graphs with
labels on the nodes.
"""

from argparse import ArgumentParser

from exercise_6.data import get_dataset
from exercise_6.bdd import ChecksForBDD


def __get_parser():
    return ArgumentParser(description="""
        One prominent application of binary decision diagrams (BDDs) is in
        representing and reasoning about graphs. In this task, we consider
        directed graphs with labels on the nodes.
    """)


def __add_dataset_argument(parser):
    parser.add_argument("dataset",
        nargs="?",
        choices=[
            "pokec",
            "congress",
        ],
    )

def __add_size_argument(parser):
    parser.add_argument("size",
        nargs="?",
        choices=[
            "small",
            "medium",
            "large"
        ],
    )


def __execute(dataset, size):
    print(f"Using dataset '{dataset}', and size '{size}'.")
    nodes, edges = get_dataset(dataset, size)
    ChecksForBDD(nodes, edges)


def pokec():
    """
    Create BDD for a the "pokec" dataset and a size provided to the program by
    the shell.
    """
    parser = __get_parser()
    __add_size_argument(parser)
    args = parser.parse_args()
    __execute("pokec", args.size)


def congress():
    """
    Create BDD for a the "congress" dataset and a size provided to the program
    by the shell.
    """
    parser = __get_parser()
    __add_size_argument(parser)
    args = parser.parse_args()
    __execute("pokec", args.size)


def main():
    """
    Create BDD for a given dataset and size. Those arguments must be provided
    to the program from the shell.
    """
    parser = __get_parser()
    __add_dataset_argument(parser)
    __add_size_argument(parser)
    args = parser.parse_args()
    __execute(args.dataset, args.size)


if __name__ == "__main__":
    main()
