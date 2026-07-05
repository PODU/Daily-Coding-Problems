# Day 1768: Direction-rule consistency: decompose each rule into strict x/y "greater-than"
# edges; a contradiction is a cycle in either axis graph (DFS cycle detection).
# Time: O(V+E) per axis, Space: O(V+E).
from collections import defaultdict


class Checker:
    def __init__(self):
        self.gx = defaultdict(list)
        self.gy = defaultdict(list)
        self.nodes = set()

    def _edge(self, g, a, b):
        g[a].append(b)
        self.nodes.add(a)
        self.nodes.add(b)

    def add_rule(self, a, direction, b):
        for c in direction:
            if c == "N":
                self._edge(self.gy, a, b)
            elif c == "S":
                self._edge(self.gy, b, a)
            elif c == "E":
                self._edge(self.gx, a, b)
            elif c == "W":
                self._edge(self.gx, b, a)

    def _has_cycle(self, g):
        state = {}

        def dfs(u):
            state[u] = 1
            for v in g[u]:
                if state.get(v, 0) == 1:
                    return True
                if state.get(v, 0) == 0 and dfs(v):
                    return True
            state[u] = 2
            return False

        return any(state.get(n, 0) == 0 and dfs(n) for n in self.nodes)

    def validates(self):
        return not self._has_cycle(self.gx) and not self._has_cycle(self.gy)


if __name__ == "__main__":
    c1 = Checker()
    c1.add_rule("A", "N", "B")
    c1.add_rule("B", "NE", "C")
    c1.add_rule("C", "N", "A")
    if not c1.validates():
        print("does not validate, since A cannot be both north and south of C.")

    c2 = Checker()
    c2.add_rule("A", "NW", "B")
    c2.add_rule("A", "N", "B")
    if c2.validates():
        print("A NW B / A N B is considered valid.")
