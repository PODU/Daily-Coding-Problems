# Day 737: Topological sort of courses (Kahn's algorithm with cycle detection).
# Lexicographic tie-break via min-heap for deterministic order.
# Time: O((V+E) log V), Space: O(V+E).
import heapq


def course_order(prereqs):
    adj = {c: [] for c in prereqs}
    indeg = {c: 0 for c in prereqs}
    for course, pres in prereqs.items():
        for p in pres:
            adj.setdefault(p, []).append(course)
            indeg[course] = indeg.get(course, 0) + 1
            indeg.setdefault(p, 0)
    heap = [c for c, d in indeg.items() if d == 0]
    heapq.heapify(heap)
    order = []
    while heap:
        c = heapq.heappop(heap)
        order.append(c)
        for nx in adj.get(c, []):
            indeg[nx] -= 1
            if indeg[nx] == 0:
                heapq.heappush(heap, nx)
    return order if len(order) == len(indeg) else None


if __name__ == "__main__":
    prereqs = {'CSC300': ['CSC100', 'CSC200'], 'CSC200': ['CSC100'], 'CSC100': []}
    print(course_order(prereqs))  # ['CSC100', 'CSC200', 'CSC300']
