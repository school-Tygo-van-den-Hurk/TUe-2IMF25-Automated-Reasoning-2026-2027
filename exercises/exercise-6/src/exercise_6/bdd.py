"""
Functions related to creating and checking things in the BDD.
"""

from math import ceil, log2

from oxidd.bdd import BDDManager

class ChecksForBDD:
    """
    Purely to keep data together that needs to stay together.
    """

    def __create_label_1_check_bdd(self):
        bdd = self.manager.false()
        for node_id, value in self.nodes.items():
            if value == 0:
                continue

            encoding = f"{node_id:0{self.encoding_length}b}"
            term = self.manager.true()

            for index in range(self.encoding_length):
                variable = self.variables[index]
                term &= variable if encoding[index] == '1' else ~variable

            bdd = bdd | term
        return bdd


    def __create_label_0_check_bdd(self):
        bdd = self.manager.false()
        for node_id, value in self.nodes.items():
            if value == 1:
                continue

            encoding = f"{node_id:0{self.encoding_length}b}"
            term = self.manager.true()

            for index in range(self.encoding_length):
                variable = self.variables[index]
                term &= variable if encoding[index] == '1' else ~variable

            bdd = bdd | term

        return bdd


    def __create_id_check_bdd(self):
        bdd = self.manager.false()

        for node_id, _ in self.nodes.items():
            term = self.manager.true()
            encoding = f"{node_id:0{self.encoding_length}b}"

            for index in range(self.encoding_length):
                variable = self.variables[index]
                term &= variable if encoding[index] == '1' else ~variable

            bdd = bdd | term

        return bdd


    def __create_transition_check_bdd(self):
        bdd = self.manager.false()

        for source, target in self.edges:
            term = self.manager.true()
            encoding_target = f"{source:0{self.encoding_length}b}"
            encoding_source = f"{target:0{self.encoding_length}b}"
            encoding = encoding_source + encoding_target

            for index in range(self.encoding_length):
                variable = self.variables[index]
                term &= variable if encoding[index] == '1' else ~variable

            bdd = bdd | term

        return bdd


    def __check_node_id(self, bdd, node_id):
        term = self.manager.true()
        encoding = f"{node_id:0{self.encoding_length}b}"

        for index in range(self.encoding_length):
            variable = self.variables[index]
            term &= variable if encoding[index] == '1' else ~variable

        return (bdd & term) != self.manager.false()


    def __check_transition(self, bdd, source, target):
        term = self.manager.true()
        encoding_target = f"{source:0{self.encoding_length}b}"
        encoding_source = f"{target:0{self.encoding_length}b}"
        encoding = encoding_source + encoding_target

        for index in range(self.encoding_length):
            variable = self.variables[index]
            term &= variable if encoding[index] == '1' else ~variable

        return (bdd & term) != self.manager.false()


    def __x_0(self, node_id):
        return self.__check_node_id(self.__bdd_0, node_id)


    def __x_1(self, node_id):
        return self.__check_node_id(self.__bdd_1, node_id)
    


    def __t(self, source, target):
        return self.__check_transition(self.__bdd_id, source, target)


    def __nodes_reachable_from1(self, unvisited_nodes):
        count = 0
        stack = [1]
        unvisited_nodes.remove(1)

        while len(stack) > 0:
            current_node = stack.pop()

            reachable_nodes = []
            for node in unvisited_nodes:
                if self.__t(current_node, node):
                    reachable_nodes.append(node)

            for node in reachable_nodes:
                stack.append(node)
                unvisited_nodes.remove(node)
                count += 1

        return count


    def __nodes_reachable_from_1_with_1_path(self, unvisited_nodes):
        count = 0
        stack = [1]
        unvisited_nodes.remove(1)

        while len(stack) > 0:
            current_node = stack.pop()

            reachable_nodes = []
            for node in unvisited_nodes:
                t = self.__t(current_node, node)
                if t and self.__x_0(current_node) != self.__x_0(node):
                    reachable_nodes.append(node)

            for node in reachable_nodes:
                stack.append(node)
                unvisited_nodes.remove(node)
                count += 1

        return count


    def __init__(self, nodes, edges):
        self.nodes = nodes
        self.edges = edges

        self.manager = BDDManager(100_000_000, 1_000_000, 1)

        self.amount_of_nodes = len(nodes)
        print(f"Amount of nodes (S): {self.amount_of_nodes}")

        self.encoding_length = ceil(log2(self.amount_of_nodes))
        print(f"Encoding length (l): {self.encoding_length}")

        variables = self.manager.add_vars(self.encoding_length)
        add = lambda variable: self.manager.var(variable)
        self.variables = [add(variable) for variable in variables]

        self.__bdd_0 = self.__create_label_0_check_bdd()
        self.__bdd_1 = self.__create_label_1_check_bdd()
        self.__bdd_id = self.__create_id_check_bdd()
        self.__bdd_transition = self.__create_transition_check_bdd()

        unvisited_nodes = set(nodes.keys())

        print("Nodes reachable:")
        print(self.__nodes_reachable_from1(unvisited_nodes))

        unvisited_nodes = set(nodes.keys())

        print("Nodes reachable by 0/1 path")
        print(self.__nodes_reachable_from_1_with_1_path(unvisited_nodes))
