# Day 1724: Cheapest itinerary with at most k connections (k flights/edges).
# Bellman-Ford limited to k relaxation rounds; track parents to reconstruct path.
# Time: O(k * E), Space: O(V).


def cheapest_itinerary(flights, src, dst, k):
    INF = float("inf")
    cities = {c for u, v, _ in flights for c in (u, v)}
    dist = {c: INF for c in cities}
    dist[src] = 0
    parent = {}

    # Relax all edges k times over a snapshot to bound edge count.
    for _ in range(k):
        snap = dict(dist)
        for u, v, w in flights:
            if snap[u] != INF and snap[u] + w < dist[v]:
                dist[v] = snap[u] + w
                parent[v] = u

    if dist[dst] == INF:
        return None, None
    path = []
    at = dst
    while True:
        path.append(at)
        if at == src:
            break
        at = parent[at]
    path.reverse()
    return path, dist[dst]


def main():
    flights = [
        ("JFK", "ATL", 150), ("ATL", "SFO", 400), ("ORD", "LAX", 200),
        ("LAX", "DFW", 80),  ("JFK", "HKG", 800), ("ATL", "ORD", 90),
        ("JFK", "LAX", 500),
    ]
    path, cost = cheapest_itinerary(flights, "JFK", "LAX", 3)
    print(f"{' -> '.join(path)}, costing ${cost}")


if __name__ == "__main__":
    main()
