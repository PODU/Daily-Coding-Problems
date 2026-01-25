# Day 958: reconstruct itinerary using every flight once, lexicographically smallest.
# Backtracking DFS over sorted adjacency. Worst O(E!) but fast in practice; Space O(E).
from collections import defaultdict


def itinerary(flights, start):
    adj = defaultdict(list)
    for o, d in flights:
        adj[o].append(d)
    for k in adj:
        adj[k].sort()
    total = len(flights)
    path = [start]

    def dfs(node):
        if len(path) == total + 1:
            return True
        dests = adj[node]
        for i in range(len(dests)):
            d = dests[i]
            if d is None:
                continue
            dests[i] = None
            path.append(d)
            if dfs(d):
                return True
            path.pop()
            dests[i] = d
        return False

    return path if dfs(start) else None


def show(v):
    return "null" if v is None else "[" + ", ".join(f"'{x}'" for x in v) + "]"


if __name__ == "__main__":
    print(show(itinerary([('SFO', 'HKO'), ('YYZ', 'SFO'), ('YUL', 'YYZ'), ('HKO', 'ORD')], 'YUL')))
    print(show(itinerary([('SFO', 'COM'), ('COM', 'YYZ')], 'COM')))
    print(show(itinerary([('A', 'B'), ('A', 'C'), ('B', 'C'), ('C', 'A')], 'A')))
