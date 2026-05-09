# Day 1494: Reverse a directed graph by reversing every edge.
# Approach: build a reversed adjacency list (for u->v add v->u). Time O(V+E), Space O(V+E).
from collections import defaultdict


def reverse_graph(edges):
    rev = defaultdict(list)
    for u, v in edges:
        rev[v].append(u)  # v -> u
    return rev


if __name__ == "__main__":
    # Input graph: A -> B -> C
    edges = [("A", "B"), ("B", "C")]
    rev = reverse_graph(edges)

    print("Original: A -> B -> C")
    print("Reversed: A <- B <- C")
    print("Reversed edges:")
    for u, v in edges:
        print(f"  {v} -> {u}")
