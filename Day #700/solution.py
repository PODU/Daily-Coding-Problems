# Day 700: Cheapest itinerary with at most k connections (k+1 flights).
# Approach: Bellman-Ford bounded to k+1 edges (relax with previous layer's dist),
# track predecessors to rebuild the route. Time O((k+1)*E), Space O(V).


def cheapest(flights, src, dst, k):
    dist = {src: 0}
    par = {}
    for _ in range(k + 1):  # up to k+1 edges
        nd, np = dict(dist), dict(par)
        for u, v, w in flights:
            if u in dist:
                cand = dist[u] + w
                if v not in nd or cand < nd[v]:
                    nd[v] = cand
                    np[v] = u
        dist, par = nd, np
    if dst not in dist:
        return -1, []
    path, cur = [], dst
    while cur != src:
        path.append(cur)
        cur = par[cur]
    path.append(src)
    return dist[dst], path[::-1]


if __name__ == "__main__":
    flights = [
        ("JFK", "ATL", 150), ("ATL", "SFO", 400), ("ORD", "LAX", 200),
        ("LAX", "DFW", 80), ("JFK", "HKG", 800), ("ATL", "ORD", 90),
        ("JFK", "LAX", 500),
    ]
    cost, path = cheapest(flights, "JFK", "LAX", 3)
    print(" -> ".join(path) + f", costing ${cost}")
    # JFK -> ATL -> ORD -> LAX, costing $440
