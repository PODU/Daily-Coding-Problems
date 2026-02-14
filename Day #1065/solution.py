# Day 1065: Reverse a directed graph (reverse every edge).
# Approach: iterate all edges u->v, add v->u to reversed adjacency. Time O(V+E), Space O(V+E).


def reverse_graph(g):
    r = {u: [] for u in g}  # keep isolated nodes
    for u, neighbors in g.items():
        for v in neighbors:
            r.setdefault(v, []).append(u)
    return r


if __name__ == "__main__":
    # A -> B -> C
    g = {"A": ["B"], "B": ["C"], "C": []}
    r = reverse_graph(g)
    # Reversed: B -> A, C -> B  ("A <- B <- C")
    print("A <- B <- C")
    for u in sorted(r):
        for v in r[u]:
            print(f"{u} -> {v}")
