# Day 1488: Topological sort of courses via Kahn's algorithm (BFS on in-degrees).
# Returns a valid ordering, or None if a cycle exists. Time O(V+E), Space O(V+E).
import heapq


def topo_sort(prereqs):
    # prereqs[course] = list of its prerequisites.
    indeg = {}
    adj = {}
    for course, ps in prereqs.items():
        indeg.setdefault(course, 0)
        for p in ps:
            indeg.setdefault(p, 0)
    for course, ps in prereqs.items():
        for p in ps:
            adj.setdefault(p, []).append(course)  # p before course
            indeg[course] += 1

    heap = [c for c, d in indeg.items() if d == 0]
    heapq.heapify(heap)  # lexicographic for deterministic output

    order = []
    while heap:
        c = heapq.heappop(heap)
        order.append(c)
        for nxt in adj.get(c, []):
            indeg[nxt] -= 1
            if indeg[nxt] == 0:
                heapq.heappush(heap, nxt)

    return order if len(order) == len(indeg) else None


if __name__ == "__main__":
    prereqs = {'CSC300': ['CSC100', 'CSC200'], 'CSC200': ['CSC100'], 'CSC100': []}
    result = topo_sort(prereqs)
    print(result if result is None else result)
