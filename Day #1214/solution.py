# Day 1214: Reconstruct itinerary using all flights, lexicographically smallest.
# Hierholzer's Eulerian path over sorted multigraph adjacency. Time O(E log E), Space O(E).
import heapq
from collections import defaultdict


def find_itinerary(flights, start):
    adj = defaultdict(list)
    for u, v in flights:
        heapq.heappush(adj[u], v)
    total = len(flights)
    route, st = [], [start]
    while st:
        u = st[-1]
        if adj[u]:
            st.append(heapq.heappop(adj[u]))
        else:
            route.append(st.pop())
    route.reverse()
    return route if len(route) == total + 1 else None


if __name__ == "__main__":
    flights = [("SFO", "HKO"), ("YYZ", "SFO"), ("YUL", "YYZ"), ("HKO", "ORD")]
    print(find_itinerary(flights, "YUL"))
    # ['YUL', 'YYZ', 'SFO', 'HKO', 'ORD']
