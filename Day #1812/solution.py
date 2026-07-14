# Day 1812: Split students into two teams so no enemies share a team = graph 2-coloring (bipartite check).
# BFS coloring over each component. Time: O(V+E). Space: O(V).
from collections import deque


def divide_teams(g):
    color = {}
    for s in g:
        if s in color:
            continue
        color[s] = 0
        q = deque([s])
        while q:
            u = q.popleft()
            for v in g[u]:
                if v not in color:
                    color[v] = color[u] ^ 1
                    q.append(v)
                elif color[v] == color[u]:
                    return False
    team_a = {n for n, c in color.items() if c == 0}
    team_b = {n for n, c in color.items() if c == 1}
    return team_a, team_b


if __name__ == "__main__":
    students = {0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3]}
    res = divide_teams(students)
    if res:
        a, b = res
        print(set(sorted(a)), "and", set(sorted(b)))
    else:
        print(False)
    # {0, 1, 4, 5} and {2, 3}
