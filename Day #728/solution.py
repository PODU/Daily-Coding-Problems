# Day 728: Split students into two teams so no enemies share a team.
# Approach: BFS 2-coloring (bipartite check). Returns two teams or False.
# Time: O(V + E), Space: O(V).
from collections import deque


def two_teams(students):
    color = {}
    for s in sorted(students):
        if s in color:
            continue
        color[s] = 0
        q = deque([s])
        while q:
            u = q.popleft()
            for v in students[u]:
                if v not in color:
                    color[v] = color[u] ^ 1
                    q.append(v)
                elif color[v] == color[u]:
                    return False
    a = sorted(k for k in students if color[k] == 0)
    b = sorted(k for k in students if color[k] == 1)
    return a, b


def show(students):
    res = two_teams(students)
    if res is False:
        print("False")
    else:
        a, b = res
        print("{" + ", ".join(map(str, a)) + "} and {" + ", ".join(map(str, b)) + "}")


if __name__ == "__main__":
    show({0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3]})
    show({0: [3], 1: [2], 2: [1, 3, 4], 3: [0, 2, 4, 5], 4: [2, 3], 5: [3]})
