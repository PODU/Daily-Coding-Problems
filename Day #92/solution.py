# Day 92: Topological sort (Kahn's algorithm) over a prerequisite graph.
# Returns an order to finish all courses, or None if a cycle exists. O(V+E).
from collections import deque


def course_order(prereqs):
    # prereqs[c] = list of courses that must come before c
    indeg = {c: 0 for c in prereqs}
    adj = {c: [] for c in prereqs}
    for course, pres in prereqs.items():
        for p in pres:
            adj.setdefault(p, [])
            adj[p].append(course)
            indeg[course] += 1
    q = deque(sorted(c for c in indeg if indeg[c] == 0))
    order = []
    while q:
        c = q.popleft()
        order.append(c)
        for nxt in sorted(adj.get(c, [])):
            indeg[nxt] -= 1
            if indeg[nxt] == 0:
                q.append(nxt)
    return order if len(order) == len(prereqs) else None


if __name__ == "__main__":
    g = {'CSC300': ['CSC100', 'CSC200'], 'CSC200': ['CSC100'], 'CSC100': []}
    print(course_order(g))  # ['CSC100', 'CSC200', 'CSC300']
