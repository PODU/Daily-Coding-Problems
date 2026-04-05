# Day 1308: Minimum cost to connect all houses to plant = Minimum Spanning Tree weight.
# Prim's algorithm with a min-heap. Time O(E log V), Space O(V + E).
import heapq


def min_cost(pipes):
    if not pipes:
        return 0
    start = next(iter(pipes))
    visited = set()
    heap = [(0, start)]
    total = 0
    while heap:
        w, u = heapq.heappop(heap)
        if u in visited:
            continue
        visited.add(u)
        total += w
        for v, cost in pipes[u].items():
            if v not in visited:
                heapq.heappush(heap, (cost, v))
    return total


def to_undirected(directed):
    g = {n: {} for n in directed}
    for a, nbrs in directed.items():
        for b, c in nbrs.items():
            g.setdefault(a, {})[b] = c
            g.setdefault(b, {})[a] = c
    return g


if __name__ == "__main__":
    pipes = {
        'plant': {'A': 1, 'B': 5, 'C': 20},
        'A': {'C': 15},
        'B': {'C': 10},
        'C': {},
    }
    print(min_cost(to_undirected(pipes)))  # 16
