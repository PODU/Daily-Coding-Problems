# Day 32: Currency arbitrage: weights = -log(rate), Bellman-Ford detects negative cycle. O(V*E).
import math


def has_arbitrage(rates):
    n = len(rates)
    w = [[-math.log(rates[i][j]) for j in range(n)] for i in range(n)]
    dist = [0.0] * n  # virtual super-source reaching all nodes at 0
    for _ in range(n - 1):
        for u in range(n):
            for v in range(n):
                if dist[u] + w[u][v] < dist[v] - 1e-12:
                    dist[v] = dist[u] + w[u][v]
    for u in range(n):
        for v in range(n):
            if dist[u] + w[u][v] < dist[v] - 1e-12:
                return True
    return False


if __name__ == "__main__":
    r1 = [[1.0, 0.7, 0.5], [1.4, 1.0, 0.7], [2.1, 1.4, 1.0]]
    r2 = [[1.0, 2.0, 4.0], [0.5, 1.0, 2.0], [0.25, 0.5, 1.0]]
    print(str(has_arbitrage(r1)).lower())
    print(str(has_arbitrage(r2)).lower())
