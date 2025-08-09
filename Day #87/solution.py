# Day 87: Validate directional rules. N/S -> vertical graph, E/W -> horizontal graph,
# edge u->v means u strictly greater on that axis. A contradiction is a cycle. Time O(V+E).
from collections import defaultdict


def has_cycle(adj):
    state = {}  # 0/absent=unvisited, 1=visiting, 2=done

    def dfs(u):
        state[u] = 1
        for v in adj[u]:
            if state.get(v, 0) == 1:
                return True
            if state.get(v, 0) == 0 and dfs(v):
                return True
        state[u] = 2
        return False

    return any(state.get(u, 0) == 0 and dfs(u) for u in list(adj))


def validate(rules):
    vert = defaultdict(set)
    horiz = defaultdict(set)
    for a, dir_, b in rules:
        for c in dir_:
            if c == "N":
                vert[a].add(b); vert[b]
            elif c == "S":
                vert[b].add(a); vert[a]
            elif c == "E":
                horiz[a].add(b); horiz[b]
            elif c == "W":
                horiz[b].add(a); horiz[a]
    return not has_cycle(vert) and not has_cycle(horiz)


if __name__ == "__main__":
    rules = [("A", "N", "B"), ("B", "NE", "C"), ("C", "N", "A")]
    print("validates" if validate(rules) else "does not validate")
    # does not validate
