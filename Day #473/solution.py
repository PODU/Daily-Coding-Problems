# Day 473: Reverse a directed graph: for each edge u->v build v->u in a new adjacency map.
# Time: O(V + E), Space: O(V + E).
from collections import defaultdict


def reverse_graph(edges):
    rev = defaultdict(list)
    for u, v in edges:
        rev[v].append(u)
    return rev


def main():
    # Original edges: A->B, B->C
    edges = [("A", "B"), ("B", "C")]
    order = ["A", "B", "C"]

    reverse_graph(edges)  # reversed adjacency map (v -> u)

    # Original chain A -> B -> C becomes A <- B <- C
    print(" <- ".join(order))


if __name__ == "__main__":
    main()
