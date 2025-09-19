# Day 297: Exact set cover via BFS/DP over bitmask of customers; each drink = bitmask of customers accepting it.
# Time: O(2^m * drinks), Space: O(2^m)  (m = number of customers, small).
from collections import deque


def min_drinks(preferences):
    customers = list(preferences.keys())
    idx = {c: i for i, c in enumerate(customers)}
    m = len(customers)
    full = (1 << m) - 1

    drink_mask = {}
    for c, drinks in preferences.items():
        for d in drinks:
            drink_mask[d] = drink_mask.get(d, 0) | (1 << idx[c])

    dp = {0: 0}
    q = deque([0])
    while q:
        mask = q.popleft()
        if mask == full:
            continue
        for dm in drink_mask.values():
            nm = mask | dm
            if nm not in dp or dp[nm] > dp[mask] + 1:
                dp[nm] = dp[mask] + 1
                q.append(nm)
    return dp[full]


if __name__ == "__main__":
    preferences = {0: [0, 1, 3, 6], 1: [1, 4, 7], 2: [2, 4, 7, 5], 3: [3, 2, 5], 4: [5, 8]}
    print(min_drinks(preferences))
