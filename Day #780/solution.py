# Day 780: Topological sort of courses (prereqs map). DFS post-order with
# cycle detection; returns None if a cycle exists. O(V + E).


def course_order(graph):
    state = {}  # 0/absent=unseen, 1=visiting, 2=done
    order = []

    def dfs(c):
        state[c] = 1
        for pre in graph.get(c, []):
            s = state.get(pre, 0)
            if s == 1:
                return False
            if s == 0 and not dfs(pre):
                return False
        state[c] = 2
        order.append(c)
        return True

    for c in sorted(graph):
        if state.get(c, 0) == 0:
            if not dfs(c):
                return None
    return order


if __name__ == "__main__":
    g = {"CSC300": ["CSC100", "CSC200"], "CSC200": ["CSC100"], "CSC100": []}
    print(course_order(g))  # ['CSC100', 'CSC200', 'CSC300']
