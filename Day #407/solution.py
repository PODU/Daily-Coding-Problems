# Day 407: Minimum Spanning Tree of water pipes (Prim's algorithm).
# Approach: Prim with a min-heap over an undirected weighted graph.
# Time: O(E log V), Space: O(V + E). Example MST total cost = 16.
import heapq


def minimum_cost(pipes):
    # Build undirected adjacency from the directed pipe dict.
    adj = {node: [] for node in pipes}
    for u, nbrs in pipes.items():
        for v, w in nbrs.items():
            adj.setdefault(u, []).append((w, v))
            adj.setdefault(v, []).append((w, u))
    if not adj:
        return 0
    start = next(iter(adj))
    visited = set()
    pq = [(0, start)]
    total = 0
    while pq:
        cost, node = heapq.heappop(pq)
        if node in visited:
            continue
        visited.add(node)
        total += cost
        for w, nbr in adj[node]:
            if nbr not in visited:
                heapq.heappush(pq, (w, nbr))
    return total


if __name__ == "__main__":
    pipes = {
        "plant": {"A": 1, "B": 5, "C": 20},
        "A": {"C": 15},
        "B": {"C": 10},
        "C": {},
    }
    print(minimum_cost(pipes))  # 16
