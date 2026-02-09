# Day 1052: Graph bipartiteness / 2-coloring via BFS over all components.
# Time O(V+E), Space O(V). Returns two teams or False if not bipartite.

from collections import deque


def divide_teams(students):
    color = {}  # node -> 0 (team A) or 1 (team B)
    for start in students:
        if start in color:
            continue
        color[start] = 0
        q = deque([start])
        while q:
            u = q.popleft()
            for v in students[u]:
                if v not in color:
                    color[v] = color[u] ^ 1
                    q.append(v)
                elif color[v] == color[u]:
                    return False
    team_a = sorted(n for n in students if color[n] == 0)
    team_b = sorted(n for n in students if color[n] == 1)
    return team_a, team_b


def format_result(res):
    if res is False:
        return "False"
    a, b = res
    return "{" + ", ".join(map(str, a)) + "} and {" + ", ".join(map(str, b)) + "}"


if __name__ == "__main__":
    students1 = {0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3]}
    students2 = {0: [3], 1: [2], 2: [1, 3, 4], 3: [0, 2, 4, 5], 4: [2, 3], 5: [3]}
    print(format_result(divide_teams(students1)))
    print(format_result(divide_teams(students2)))
