"""
Functions related to getting data out of the data directory.
"""

from pathlib import Path

from pandas import read_csv

__EXERCISE_ROOT = Path(__file__).resolve().parent.parent.parent
__DATA_DIRECTORY = __EXERCISE_ROOT / "data"
__POKEC_DIRECTORY = __DATA_DIRECTORY / "pokec"
pokec_edges_path = __DATA_DIRECTORY / "edges.csv"
pokec_nodes_path = __DATA_DIRECTORY / "nodes.csv"


paths = {
    "pokec": {
        "small": {
            "edges": __POKEC_DIRECTORY / "small" / "edges.csv",
            "nodes": __POKEC_DIRECTORY / "small" / "nodes.csv",
        },
        "medium": {
            "edges": __POKEC_DIRECTORY / "medium" / "edges.csv",
            "nodes": __POKEC_DIRECTORY / "medium" / "nodes.csv",
        },
        "large": {
            "edges": __POKEC_DIRECTORY / "large" / "edges.csv",
            "nodes": __POKEC_DIRECTORY / "large" / "nodes.csv",
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
