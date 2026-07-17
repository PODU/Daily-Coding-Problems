# Day 1825: Word circle = Eulerian circuit in graph where each word is an edge first->last char.
# Check in==out degrees, then Hierholzer to build chain. O(V+E).
from collections import defaultdict


def solve(words):
    adj = defaultdict(list)
    indeg = defaultdict(int)
    outdeg = defaultdict(int)
    for w in words:
        a, b = w[0], w[-1]
        adj[a].append((w, b))
        outdeg[a] += 1
        indeg[b] += 1

    nodes = set(indeg) | set(outdeg)
    for c in nodes:
        if indeg[c] != outdeg[c]:
            return "No circle"

    idx = defaultdict(int)
    start = words[0][0]
    st = [start]
    edge_stack = []
    circuit = []
    while st:
        u = st[-1]
        if idx[u] < len(adj[u]):
            w, v = adj[u][idx[u]]
            idx[u] += 1
            st.append(v)
            edge_stack.append(w)
        else:
            st.pop()
            if edge_stack:
                circuit.append(edge_stack.pop())

    if len(circuit) != len(words):
        return "No circle"
    circuit.reverse()
    return " --> ".join(circuit) + " --> " + circuit[0]


if __name__ == "__main__":
    words = ["chair", "height", "racket", "touch", "tunic"]
    print(solve(words))
