# Day 1344: Bipartite 2-coloring via BFS over all components (sorted iteration for determinism).
# Time: O(V+E), Space: O(V+E).
from collections import deque


def solve(students):
    color = {}
    for start in sorted(students):
        if start in color:
            continue
        color[start] = 0
        q = deque([start])
        while q:
            u = q.popleft()
            for v in sorted(students.get(u, [])):
                if v not in color:
                    color[v] = color[u] ^ 1
                    q.append(v)
                elif color[v] == color[u]:
                    return False
    t0 = sorted(k for k, c in color.items() if c == 0)
    t1 = sorted(k for k, c in color.items() if c == 1)
    return t0, t1


def group(g):
    return "{" + ", ".join(str(x) for x in g) + "}"


def main():
    s1 = {0: [3], 1: [2], 2: [1, 4], 3: [0, 4, 5], 4: [2, 3], 5: [3]}
    r1 = solve(s1)
    print(f"{group(r1[0])} and {group(r1[1])}" if r1 else "False")

    s2 = {0: [3], 1: [2], 2: [1, 3, 4], 3: [0, 2, 4, 5], 4: [2, 3], 5: [3]}
    r2 = solve(s2)
    print(f"{group(r2[0])} and {group(r2[1])}" if r2 else "False")


if __name__ == "__main__":
    main()
