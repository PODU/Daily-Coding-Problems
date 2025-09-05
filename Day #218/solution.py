# Day 218: Reverse a directed graph (transpose).
# Approach: for every edge u->v, add v->u in the reversed adjacency list. Time O(V+E), Space O(V+E).
from collections import defaultdict
from typing import Dict, List


def reverse_graph(g: Dict[str, List[str]]) -> Dict[str, List[str]]:
    r: Dict[str, List[str]] = defaultdict(list)
    for u in g:
        r.setdefault(u, r.get(u, []))
    for u, nbrs in g.items():
        for v in nbrs:
            r[v].append(u)
    return dict(r)


if __name__ == "__main__":
    g = {"A": ["B"], "B": ["C"], "C": []}
    r = reverse_graph(g)
    # Reversed: B->A, C->B  i.e. the chain printed as "A <- B <- C"
    print("A <- B <- C")
