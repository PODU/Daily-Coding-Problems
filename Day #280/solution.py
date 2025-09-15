# Day 280: Detect cycle in undirected graph via Union-Find.
# An edge joining two already-connected vertices implies a cycle.
# Time O(V + E * alpha(V)), Space O(V).


def has_cycle(n, edges):
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    for u, v in edges:
        ru, rv = find(u), find(v)
        if ru == rv:
            return True
        parent[ru] = rv
    return False


if __name__ == "__main__":
    print(has_cycle(4, [(0, 1), (1, 2), (2, 0), (2, 3)]))  # True
    print(has_cycle(4, [(0, 1), (1, 2), (2, 3)]))          # False
