# Day 439: Reconstruct itinerary using all flights, lexicographically smallest.
# Hierholzer's Eulerian-path algorithm with min-heap adjacency. O(E log E).
import heapq
from collections import defaultdict


def find_itinerary(flights, start):
    graph = defaultdict(list)
    for origin, dest in flights:
        heapq.heappush(graph[origin], dest)

    route = []
    stack = [start]
    while stack:
        u = stack[-1]
        if graph[u]:
            stack.append(heapq.heappop(graph[u]))
        else:
            route.append(stack.pop())
    route.reverse()
    if len(route) != len(flights) + 1:
        return None  # no valid itinerary using all flights
    return route


if __name__ == "__main__":
    flights = [('SFO', 'HKO'), ('YYZ', 'SFO'), ('YUL', 'YYZ'), ('HKO', 'ORD')]
    print(find_itinerary(flights, 'YUL'))
    # ['YUL', 'YYZ', 'SFO', 'HKO', 'ORD']
    print(find_itinerary([('SFO', 'COM'), ('COM', 'YYZ')], 'COM'))  # None
    print(find_itinerary([('A', 'B'), ('A', 'C'), ('B', 'C'), ('C', 'A')], 'A'))
    # ['A', 'B', 'C', 'A', 'C']
