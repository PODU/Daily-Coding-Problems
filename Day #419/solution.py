# Day 419: Min steps to reduce N to 1 (decrement, or jump to larger factor).
# BFS over values 1..N. Time: O(N*sqrt(N)), Space: O(N).
from collections import deque


def min_steps(N):
    if N <= 1:
        return 0
    dist = [-1] * (N + 1)
    dist[N] = 0
    q = deque([N])
    while q:
        v = q.popleft()
        if v == 1:
            return dist[1]
        if v - 1 >= 1 and dist[v - 1] == -1:
            dist[v - 1] = dist[v] + 1
            q.append(v - 1)
        a = 2
        while a * a <= v:
            if v % a == 0:
                larger = v // a
                if dist[larger] == -1:
                    dist[larger] = dist[v] + 1
                    q.append(larger)
            a += 1
    return dist[1]


if __name__ == "__main__":
    N = 100
    print(f"{min_steps(N)}  (route: 100 -> 10 -> 9 -> 3 -> 2 -> 1)")
