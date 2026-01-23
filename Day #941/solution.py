# Day 941: Arbitrage exists iff a cycle has product of rates > 1, i.e. a negative cycle
# under weights w = -log(rate). Bellman-Ford. Time O(V^3), Space O(V).
import math


def has_arbitrage(rate):
    n = len(rate)
    dist = [0.0] * n  # virtual source connected to all with weight 0
    for _ in range(n - 1):
        for u in range(n):
            for v in range(n):
                w = -math.log(rate[u][v])
                if dist[u] + w < dist[v] - 1e-12:
                    dist[v] = dist[u] + w
    for u in range(n):
        for v in range(n):
            w = -math.log(rate[u][v])
            if dist[u] + w < dist[v] - 1e-12:
                return True
    return False


if __name__ == "__main__":
    rate = [
        [1.0, 2.0, 1.0],
        [0.5, 1.0, 2.0],
        [1.0, 0.5, 1.0],
    ]
    print(has_arbitrage(rate))  # True (0->1->2->0 = 4 > 1)
