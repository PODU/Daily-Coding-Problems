# Day 1828: Arbitrage = negative cycle in graph with weights -log(rate). Bellman-Ford.
# O(V^3) for a dense rate table.
import math


def has_arbitrage(rate):
    n = len(rate)
    w = [[-math.log(rate[i][j]) for j in range(n)] for i in range(n)]
    dist = [0.0] * n  # virtual source reaching every node with 0
    for _ in range(n - 1):
        for u in range(n):
            for v in range(n):
                if dist[u] + w[u][v] < dist[v]:
                    dist[v] = dist[u] + w[u][v]
    for u in range(n):
        for v in range(n):
            if dist[u] + w[u][v] < dist[v] - 1e-12:
                return True
    return False


if __name__ == "__main__":
    # 0.8 * 1.3 = 1.04 > 1 => arbitrage exists
    rate = [
        [1.0, 0.8, 0.5],
        [1.3, 1.0, 1.9],
        [1.9, 0.5, 1.0],
    ]
    print("true" if has_arbitrage(rate) else "false")  # true
