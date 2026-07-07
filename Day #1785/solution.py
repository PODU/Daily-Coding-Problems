# Day 1785: Build origin->sorted destinations; DFS backtracking trying lexicographically
# smallest dest first; first path using all flights is the answer.
# Time O(E!) worst case, Space O(E).
from collections import defaultdict

def find_itinerary(flights, start):
    total = len(flights)
    graph = defaultdict(list)
    for o, d in flights:
        graph[o].append(d)
    for o in graph:
        graph[o].sort()

    def dfs(node, path):
        if len(path) == total + 1:
            return list(path)
        for i, d in enumerate(graph[node]):
            if d is None:
                continue
            graph[node][i] = None  # mark used
            path.append(d)
            res = dfs(d, path)
            if res is not None:
                return res
            path.pop()
            graph[node][i] = d
        return None

    return dfs(start, [start])

def show(r):
    print("null" if r is None else r)

if __name__ == '__main__':
    show(find_itinerary([('SFO','HKO'),('YYZ','SFO'),('YUL','YYZ'),('HKO','ORD')], 'YUL'))
    show(find_itinerary([('SFO','COM'),('COM','YYZ')], 'COM'))
    show(find_itinerary([('A','B'),('A','C'),('B','C'),('C','A')], 'A'))
