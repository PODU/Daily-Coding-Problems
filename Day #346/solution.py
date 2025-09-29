# Day 346: Cheapest itinerary with up to k connections.
# Bellman-Ford limited to k+1 edges, tracking the path. Time O(k*E), Space O(V).
from collections import defaultdict


def cheapest_itinerary(flights, src, dst, k):
    # dist[node] = (cost, path) using the best route found so far.
    best = {src: (0, [src])}
    for _ in range(k + 1):  # up to k connections => k+1 flights
        nxt = dict(best)
        for u, v, w in flights:
            if u in best:
                cost = best[u][0] + w
                if v not in nxt or cost < nxt[v][0]:
                    nxt[v] = (cost, best[u][1] + [v])
        best = nxt
    if dst not in best:
        return None
    return best[dst]


def main():
    flights = [
        ('JFK', 'ATL', 150),
        ('ATL', 'SFO', 400),
        ('ORD', 'LAX', 200),
        ('LAX', 'DFW', 80),
        ('JFK', 'HKG', 800),
        ('ATL', 'ORD', 90),
        ('JFK', 'LAX', 500),
    ]
    cost, path = cheapest_itinerary(flights, 'JFK', 'LAX', 3)
    print("{}, costing ${}.".format(" -> ".join(path), cost))


if __name__ == "__main__":
    main()
