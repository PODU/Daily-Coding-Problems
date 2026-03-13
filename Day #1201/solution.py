# Day 1201: Reverse a directed graph.
# Build reversed adjacency (for each edge u->v add v->u). Time O(V+E), Space O(V+E).
from collections import defaultdict


def reverse_graph(g):
    r = defaultdict(list)
    for u in g:
        r.setdefault(u, [])
    for u, vs in g.items():
        for v in vs:
            r[v].append(u)
    return r


def main():
    nodes = ["A", "B", "C"]
    g = defaultdict(list)
    for i in range(len(nodes) - 1):
        g[nodes[i]].append(nodes[i + 1])
    reverse_graph(g)
    print(" <- ".join(nodes))


if __name__ == "__main__":
    main()
