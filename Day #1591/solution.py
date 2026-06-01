# Day 1591: Subset Sum: boolean DP over reachable sums; reconstruct one subset by backtracking.
# Time O(n*k), Space O(n*k).
from typing import List, Optional


def subset_sum(S: List[int], k: int) -> Optional[List[int]]:
    n = len(S)
    reach = [[False] * (k + 1) for _ in range(n + 1)]
    reach[0][0] = True
    for i in range(1, n + 1):
        for s in range(k + 1):
            if reach[i - 1][s]:
                reach[i][s] = True
            if s >= S[i - 1] and reach[i - 1][s - S[i - 1]]:
                reach[i][s] = True
    if not reach[n][k]:
        return None
    chosen = []
    s = k
    for i in range(n, 0, -1):
        if s >= S[i - 1] and reach[i - 1][s - S[i - 1]]:
            chosen.append(S[i - 1])
            s -= S[i - 1]
    return chosen


if __name__ == "__main__":
    S = [12, 1, 61, 5, 9, 2]
    k = 24
    sub = subset_sum(S, k)
    print(sub)
    print("Sum =", sum(sub) if sub else 0)
