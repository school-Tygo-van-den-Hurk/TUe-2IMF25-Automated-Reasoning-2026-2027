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


__dataset_options = ["pokec", "congress"]
def __add_dataset_argument(parser):
    parser.add_argument("dataset",
        nargs="?",
        choices=__dataset_options,
    )


__size_options = [ "small", "medium", "large" ]
def __add_size_argument(parser):
    parser.add_argument("size",
        nargs="?",
        choices=__size_options,
    )


def __ask_user_to_pick(options, question):
    print(question)
    print(f"Choose from: {', '.join(options)}")
    request = input("> ")
    if request in options:
        return request
    print("That is not one of the options")


def execute(dataset, size):
    """
    Execute the program for a given dataset and size. If dataset and size are
    not as expected, it will fall back to asking the user instead.
    """
    try:
        while dataset not in __dataset_options:
            dataset = __ask_user_to_pick(
                question="What dataset would you like to investigate?",
                options=__dataset_options,
            )

        while size not in __size_options:
            size = __ask_user_to_pick(
                question="For what size would you like to run?",
                options=__size_options,
            )

        print(f"Using dataset '{dataset}', and size '{size}'.")
        nodes, edges = get_dataset(dataset, size)
        ChecksForBDD(nodes, edges)

    except KeyboardInterrupt:
        print("\nerror: keyboard interruption")
        exit(1)


def pokec():
    """
    Create BDD for a the "pokec" dataset and a size provided to the program by
    the shell, otherwise it will fall back to asking the user.
    """
    parser = __get_parser()
    __add_size_argument(parser)
    args = parser.parse_args()
    execute("pokec", args.size)


def congress():
    """
    Create BDD for a the "congress" dataset and a size provided to the program
    by the shell, otherwise it will fall back to asking the user.
    """
    parser = __get_parser()
    __add_size_argument(parser)
    args = parser.parse_args()
    execute("congress", args.size)


def main():
    """
    Create BDD for a given dataset and size. Those arguments must be provided
    to the program from the shell, otherwise it will fall back to asking the
    user.
    """
    parser = __get_parser()
    __add_dataset_argument(parser)
    __add_size_argument(parser)
    args = parser.parse_args()
    execute(args.dataset, args.size)


if __name__ == "__main__":
    main()
