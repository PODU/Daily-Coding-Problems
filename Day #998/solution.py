# Day 998: Graph k-colorability (adjacency matrix).
# Backtracking: try each color for each vertex, skipping colors used by an
# adjacent vertex. O(k^V) worst case, O(V) extra space.

def can_color(graph, k):
    n = len(graph)
    colors = [0] * n  # 0 = uncolored, 1..k = color

    def ok(v, c):
        return all(not (graph[v][u] and colors[u] == c) for u in range(n))

    def solve(v):
        if v == n:
            return True
        for c in range(1, k + 1):
            if ok(v, c):
                colors[v] = c
                if solve(v + 1):
                    return True
                colors[v] = 0
        return False

    return solve(0)


if __name__ == "__main__":
    triangle = [[0, 1, 1], [1, 0, 1], [1, 1, 0]]
    print(can_color(triangle, 2))  # False
    print(can_color(triangle, 3))  # True
