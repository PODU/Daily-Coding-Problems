# Day 1053: Directional consistency. Decompose each rule into strict x/y
# inequalities, build a directed "greater-than" graph per axis, and detect a
# cycle (contradiction) via DFS. Time O(V+E), Space O(V+E).

def validate(rules):
    # axis graphs: edge u -> v means u > v on that axis
    gx, gy = {}, {}

    def add(g, u, v):
        g.setdefault(u, set()).add(v)
        g.setdefault(v, set())

    for rule in rules:
        a, d, b = rule.split()
        for ch in d:
            if ch == 'N':   add(gy, a, b)   # a.y > b.y
            elif ch == 'S': add(gy, b, a)
            elif ch == 'E': add(gx, a, b)   # a.x > b.x
            elif ch == 'W': add(gx, b, a)

    return not (has_cycle(gx) or has_cycle(gy))


def has_cycle(g):
    state = {}  # 0=visiting, 1=done

    def dfs(u):
        state[u] = 0
        for w in g.get(u, ()):
            if state.get(w) == 0:
                return True
            if w not in state and dfs(w):
                return True
        state[u] = 1
        return False

    return any(n not in state and dfs(n) for n in g)


if __name__ == "__main__":
    ex1 = ["A N B", "B NE C", "C N A"]
    ex2 = ["A NW B", "A N B"]
    print("does not validate, since A cannot be both north and south of C."
          if not validate(ex1) else "is considered valid.")
    print("is considered valid." if validate(ex2)
          else "does not validate.")
