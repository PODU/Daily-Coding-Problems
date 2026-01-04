# Day 851: a graph is minimally-connected iff it is a tree: connected AND edges == nodes-1.
# Union-Find detects cycles and connectivity in O(E alpha(V)).

def is_minimally_connected(n, edges):
    if len(edges) != n - 1:
        return False
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for a, b in edges:
        ra, rb = find(a), find(b)
        if ra == rb:
            return False  # cycle
        parent[ra] = rb
    roots = {find(i) for i in range(n)}
    return len(roots) == 1


if __name__ == "__main__":
    print(is_minimally_connected(5, [(0, 1), (0, 2), (1, 3), (1, 4)]))  # True
    print(is_minimally_connected(4, [(0, 1), (1, 2), (2, 0), (2, 3)]))  # False
