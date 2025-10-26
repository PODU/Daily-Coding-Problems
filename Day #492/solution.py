# Day 492: Graph m-colorability via backtracking.
# Assign each vertex a color 1..k, ensuring no adjacent pair (adjacency matrix) matches.
# Time: O(k^V) worst case, Space: O(V) for the color assignment + recursion stack.


def can_color(graph, k):
    n = len(graph)
    colors = [0] * n

    def is_safe(v, c):
        return all(not (graph[v][u] and colors[u] == c) for u in range(n))

    def solve(v):
        if v == n:
            return True
        for c in range(1, k + 1):
            if is_safe(v, c):
                colors[v] = c
                if solve(v + 1):
                    return True
                colors[v] = 0
        return False

    return solve(0)


if __name__ == "__main__":
    # Triangle K3: every pair adjacent.
    graph = [
        [0, 1, 1],
        [1, 0, 1],
        [1, 1, 0],
    ]
    print("k=2 colorable:", str(can_color(graph, 2)).lower())
    print("k=3 colorable:", str(can_color(graph, 3)).lower())
