"""
Functions related to getting data out of the data directory.
"""

from os import getenv
from pathlib import Path

from pandas import read_csv

__EXERCISE_ROOT = Path(__file__).resolve().parent.parent.parent
__DATA_DIRECTORY_FALLBACK = __EXERCISE_ROOT / "data"
__DATA_DIRECTORY_ENV = getenv("EXERCISE_6_DATA_DIR", __DATA_DIRECTORY_FALLBACK)
__DATA_DIRECTORY = Path(__DATA_DIRECTORY_ENV).resolve()


paths = {
    "pokec": {
        "small": {
            "edges": __DATA_DIRECTORY / "pokec" / "small" / "edges.csv",
            "nodes": __DATA_DIRECTORY / "pokec" / "small" / "nodes.csv",
        },
        "medium": {
            "edges": __DATA_DIRECTORY / "pokec" / "medium" / "edges.csv",
            "nodes": __DATA_DIRECTORY / "pokec" / "medium" / "nodes.csv",
        },
        "large": {
            "edges": __DATA_DIRECTORY / "pokec" / "large" / "edges.csv",
            "nodes": __DATA_DIRECTORY / "pokec" / "large" / "nodes.csv",
        },
    },
    "congress": {
        "small": {
            "edges": __DATA_DIRECTORY / "congress" / "small" / "edges.csv",
            "nodes": __DATA_DIRECTORY / "congress" / "small" / "nodes.csv",
        },
        "medium": {
            "edges": __DATA_DIRECTORY / "congress" / "medium" / "edges.csv",
            "nodes": __DATA_DIRECTORY / "congress" / "medium" / "nodes.csv",
        },
        "large": {
            "edges": __DATA_DIRECTORY / "congress" / "large" / "edges.csv",
            "nodes": __DATA_DIRECTORY / "congress" / "large" / "nodes.csv",
        },
    },
}


def __get_nodes_pokec(path, nodes=None):
    """
    Parses the CSV at the specified path returning it's nodes.
    """

    if nodes is None:
        nodes = dict()

    data = read_csv(path, sep='\t')
    for row in data.itertuples(index=False):
        nodes[row.id] = row.label

    return nodes


def __get_edges_pokec(path, transitions=None):
    """
    Parses the CSV at the specified path returning it's transitions.
    """

    if transitions is None:
        transitions = set()

    data = read_csv(path, sep='\t')
    for row in data.itertuples(index=False):
        source = int(row.source)
        target = int(row.target)
        transitions.add((source, target))

    return transitions


def get_dataset(name, size):
    """
    Gets a dataset by name and size.
    """
    if name == "pokec":
        path = paths["pokec"][size]["nodes"]
        nodes = __get_nodes_pokec(path)
        path = paths["pokec"][size]["edges"]
        edges = __get_edges_pokec(path)
        return nodes, edges

    else: raise ValueError("No such dataset")
