# Day 246: Words form a circle: model each word as a directed edge first->last char; an Eulerian circuit
# exists iff in-degree==out-degree for every node and edges form one connected component.
# Find the circuit with Hierholzer's algorithm. O(V + E) time and space.
from collections import defaultdict


def circle_order(words):
    adj = defaultdict(list)  # first char -> list of (last char, word)
    indeg = defaultdict(int)
    outdeg = defaultdict(int)
    nodes = set()
    for w in words:
        a, b = w[0], w[-1]
        adj[a].append((b, w))
        outdeg[a] += 1
        indeg[b] += 1
        nodes.add(a)
        nodes.add(b)

    for c in nodes:
        if indeg[c] != outdeg[c]:
            return None

    # Connectivity check (undirected) over nodes that have outgoing edges.
    und = defaultdict(list)
    for a, lst in adj.items():
        for b, _ in lst:
            und[a].append(b)
            und[b].append(a)
    active = {a for a, lst in adj.items() if lst}
    if not active:
        return None
    seen = set()
    stack = [next(iter(active))]
    while stack:
        c = stack.pop()
        if c in seen:
            continue
        seen.add(c)
        for nb in und[c]:
            if nb not in seen:
                stack.append(nb)
    if any(c not in seen for c in active):
        return None

    # Hierholzer starting from the first word's first char.
    start = words[0][0]
    ptr = defaultdict(int)
    node_stack = [start]
    edge_stack = []
    circuit = []
    while node_stack:
        v = node_stack[-1]
        if ptr[v] < len(adj[v]):
            nxt, word = adj[v][ptr[v]]
            ptr[v] += 1
            node_stack.append(nxt)
            edge_stack.append(word)
        else:
            node_stack.pop()
            if edge_stack:
                circuit.append(edge_stack.pop())
    if len(circuit) != len(words):
        return None
    circuit.reverse()
    return circuit


def main():
    words = ["chair", "height", "racket", "touch", "tunic"]
    order = circle_order(words)
    if order:
        print(" --> ".join(order) + " --> " + order[0])
    else:
        print("No circle possible")


if __name__ == "__main__":
    main()
