# Day 56: Graph k-colorability via backtracking.
# Time: O(k^V) worst case, Space: O(V).


def k_colorable(g, k):
    n = len(g)
    color = [0] * n

    def can_color(v):
        if v == n:
            return True
        for c in range(1, k + 1):
            if all(not (g[v][u] and color[u] == c) for u in range(n)):
                color[v] = c
                if can_color(v + 1):
                    return True
                color[v] = 0
        return False

    return can_color(0)


if __name__ == "__main__":
    # Triangle graph: needs 3 colors.
    g = [
        [0, 1, 1],
        [1, 0, 1],
        [1, 1, 0],
    ]
    print(k_colorable(g, 2))  # False
    print(k_colorable(g, 3))  # True
