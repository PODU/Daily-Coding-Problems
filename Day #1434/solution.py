# Day 1434: Cheapest flight A->B with at most k connections; print itinerary.
# Approach: Bellman-Ford relaxed k+1 times (k connections = k+1 edges), track parent.
# Time: O((k+1) * E), Space: O(V).
import math


def cheapest(flights, A, B, k):
    dist = {}
    for s, d, _ in flights:
        dist.setdefault(s, math.inf)
        dist.setdefault(d, math.inf)
    dist[A] = 0
    parent = {}
    for _ in range(k + 1):  # k connections -> at most k+1 edges
        cur = dict(dist)
        cur_parent = dict(parent)
        for s, d, price in flights:
            if dist[s] != math.inf and dist[s] + price < cur[d]:
                cur[d] = dist[s] + price
                cur_parent[d] = s
        dist, parent = cur, cur_parent
    if dist[B] == math.inf:
        return "No route"
    path = []
    node = B
    while node != A:
        path.append(node)
        node = parent[node]
    path.append(A)
    path.reverse()
    return " -> ".join(path) + ", costing $" + str(dist[B])


if __name__ == "__main__":
    flights = [
        ('JFK', 'ATL', 150), ('ATL', 'SFO', 400), ('ORD', 'LAX', 200),
        ('LAX', 'DFW', 80), ('JFK', 'HKG', 800), ('ATL', 'ORD', 90), ('JFK', 'LAX', 500),
    ]
    print(cheapest(flights, 'JFK', 'LAX', 3))
