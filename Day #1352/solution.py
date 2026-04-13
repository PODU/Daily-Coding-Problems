# Day 1352: Arbitrage detection: edge weight = -log(rate); negative cycle => arbitrage. Bellman-Ford.
# Time: O(V*E) = O(V^3), Space: O(V).
import math


def has_arbitrage(rates):
    n = len(rates)
    dist = [0.0] * n  # all 0: detect any reachable negative cycle
    for _ in range(n - 1):
        for u in range(n):
            for v in range(n):
                w = -math.log(rates[u][v])
                if dist[u] + w < dist[v] - 1e-12:
                    dist[v] = dist[u] + w
    for u in range(n):
        for v in range(n):
            w = -math.log(rates[u][v])
            if dist[u] + w < dist[v] - 1e-12:
                return True
    return False


if __name__ == "__main__":
    rates = [
        [1.0, 2.0, 1.0],
        [0.5, 1.0, 4.0],
        [1.0, 0.25, 1.0],
    ]
    print(f"Arbitrage exists: {str(has_arbitrage(rates)).lower()}")
