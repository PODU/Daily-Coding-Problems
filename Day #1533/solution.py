# Day 1533: Minimum spanning tree (Prim's algorithm) over an undirected pipe graph.
# Returns total cost to connect every house to the plant.
# Time O(E log V), Space O(V + E).
import heapq
from collections import defaultdict


def mst_cost(pipes):
    adj = defaultdict(list)
    for u, nbrs in pipes.items():
        adj[u]  # ensure node exists
        for v, w in nbrs.items():
            adj[u].append((w, v))
            adj[v].append((w, u))
    start = next(iter(pipes))
    visited = set()
    pq = [(0, start)]
    total = 0
    while pq:
        w, u = heapq.heappop(pq)
        if u in visited:
            continue
        visited.add(u)
        total += w
        for cw, v in adj[u]:
            if v not in visited:
                heapq.heappush(pq, (cw, v))
    return total


if __name__ == "__main__":
    pipes = {
        'plant': {'A': 1, 'B': 5, 'C': 20},
        'A': {'C': 15},
        'B': {'C': 10},
        'C': {}
    }
    print(mst_cost(pipes))
