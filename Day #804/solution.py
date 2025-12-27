# Day 804: Cheapest fare from A to B with <= k connections (<= k+1 flights).
# Bellman-Ford limited to k+1 relaxation rounds, tracking parents for itinerary.
# Time O((k+1) * E), Space O(V + E).
import math


def cheapest(flights, A, B, k):
    dist = {}
    for s, d, _ in flights:
        dist[s] = math.inf
        dist[d] = math.inf
    dist[A] = 0
    parent = {}
    for _ in range(k + 1):  # k+1 edges allowed
        snap = dict(dist)
        for s, d, p in flights:
            if snap[s] + p < dist[d]:
                dist[d] = snap[s] + p
                parent[d] = s
    if dist[B] == math.inf:
        return -1, []
    path = []
    c = B
    while True:
        path.append(c)
        if c == A:
            break
        c = parent[c]
    return dist[B], path[::-1]


if __name__ == "__main__":
    flights = [
        ("JFK", "ATL", 150), ("ATL", "SFO", 400), ("ORD", "LAX", 200),
        ("LAX", "DFW", 80), ("JFK", "HKG", 800), ("ATL", "ORD", 90),
        ("JFK", "LAX", 500),
    ]
    cost, path = cheapest(flights, "JFK", "LAX", 3)
    print(f"{' -> '.join(path)}, costing ${cost}")
    # JFK -> ATL -> ORD -> LAX, costing $440
