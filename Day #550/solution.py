# Day 550: Arbitrage detection: weight=-log(rate), Bellman-Ford finds a negative-weight cycle => arbitrage. O(V^3) (V*E edges, V-1 passes).
import math


def has_arbitrage(rates):
    n = len(rates)
    dist = [0.0] * n  # virtual source: all start at 0
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
    arb = [[1, 0.5, 0.2], [2, 1, 0.5], [5, 2, 1]]
    consistent = [[1, 2, 4], [0.5, 1, 2], [0.25, 0.5, 1]]
    print("true" if has_arbitrage(arb) else "false")
    print("true" if has_arbitrage(consistent) else "false")
