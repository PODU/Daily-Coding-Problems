# Day 1227: Graph k-colorability via backtracking: assign colors 1..k to vertices in order,
# skipping conflicts. Time O(k^n) worst case, Space O(n).


def is_k_colorable(g, k):
    n = len(g)
    color = [0] * n

    def safe(v, c):
        return all(not (g[v][i] and color[i] == c) for i in range(n))

    def colorize(v):
        if v == n:
            return True
        for c in range(1, k + 1):
            if safe(v, c):
                color[v] = c
                if colorize(v + 1):
                    return True
                color[v] = 0
        return False

    return colorize(0)


if __name__ == "__main__":
    g = [[0, 1, 1], [1, 0, 1], [1, 1, 0]]
    print(str(is_k_colorable(g, 2)).lower())
    print(str(is_k_colorable(g, 3)).lower())
