# Day 41: Reconstruct Itinerary: backtracking DFS over lexicographically sorted edges.
# First complete itinerary using all edges (tried in lex order) is the answer.
# Time: exponential worst case; Space: O(E).

def reconstruct(flights, start):
    adj = {}
    for a, b in flights:
        adj.setdefault(a, []).append(b)
    for k in adj:
        adj[k].sort()
    used = {k: [False] * len(v) for k, v in adj.items()}
    total = len(flights)
    path = [start]

    def dfs(node):
        if len(path) == total + 1:
            return True
        dests = adj.get(node)
        if dests is None:
            return False
        u = used[node]
        for i in range(len(dests)):
            if u[i]:
                continue
            u[i] = True
            path.append(dests[i])
            if dfs(dests[i]):
                return True
            path.pop()
            u[i] = False
        return False

    return path if dfs(start) else None


def fmt(v):
    if v is None:
        return "null"
    return "[" + ", ".join("'" + x + "'" for x in v) + "]"


if __name__ == "__main__":
    print(fmt(reconstruct([('SFO','HKO'),('YYZ','SFO'),('YUL','YYZ'),('HKO','ORD')], 'YUL')))
    print(fmt(reconstruct([('SFO','COM'),('COM','YYZ')], 'COM')))
    print(fmt(reconstruct([('A','B'),('A','C'),('B','C'),('C','A')], 'A')))
