# Day 1667: Course ordering via topological sort (Kahn's algorithm).
# Time O(V+E), Space O(V+E). Returns None if a cycle exists.
import heapq
from collections import defaultdict


def course_order(prereqs):
    indeg = {}
    adj = defaultdict(list)
    for course, deps in prereqs.items():
        indeg.setdefault(course, 0)
        for d in deps:
            indeg.setdefault(d, 0)
    for course, deps in prereqs.items():
        for d in deps:
            adj[d].append(course)
            indeg[course] += 1
    heap = [c for c, d in indeg.items() if d == 0]
    heapq.heapify(heap)
    order = []
    while heap:
        c = heapq.heappop(heap)
        order.append(c)
        for nxt in adj[c]:
            indeg[nxt] -= 1
            if indeg[nxt] == 0:
                heapq.heappush(heap, nxt)
    return order if len(order) == len(indeg) else None


if __name__ == "__main__":
    g = {"CSC300": ["CSC100", "CSC200"], "CSC200": ["CSC100"], "CSC100": []}
    print(course_order(g))  # ['CSC100', 'CSC200', 'CSC300']
