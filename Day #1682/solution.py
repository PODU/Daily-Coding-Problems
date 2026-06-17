# Day 1682: Graph k-colorability via backtracking with pruning.
# Time O(k^V) worst case, Space O(V).


def can_color(adj, k):
    n = len(adj)
    color = [-1] * n

    def solve(v):
        if v == n:
            return True
        for c in range(k):
            if all(not (adj[v][u] and color[u] == c) for u in range(n)):
                color[v] = c
                if solve(v + 1):
                    return True
                color[v] = -1
        return False

    return solve(0)


if __name__ == "__main__":
    tri = [[0, 1, 1], [1, 0, 1], [1, 1, 0]]
    print(can_color(tri, 2))  # False
    print(can_color(tri, 3))  # True
