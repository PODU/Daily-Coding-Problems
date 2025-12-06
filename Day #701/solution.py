# Day 701: Minimum drinks to satisfy every customer (minimum set cover).
# Approach: each drink -> bitmask of customers it satisfies; DP over customer
# masks for fewest drinks covering all. Time O(2^C * D), Space O(2^C).


def min_drinks(prefs):
    custs = list(prefs.keys())
    idx = {c: i for i, c in enumerate(custs)}
    C = len(custs)
    drink_mask = {}
    for cust, drinks in prefs.items():
        for d in drinks:
            drink_mask[d] = drink_mask.get(d, 0) | (1 << idx[cust])
    full = (1 << C) - 1
    INF = float("inf")
    dp = [INF] * (full + 1)
    dp[0] = 0
    for mask in range(full + 1):
        if dp[mask] == INF:
            continue
        for dm in drink_mask.values():
            nm = mask | dm
            if dp[mask] + 1 < dp[nm]:
                dp[nm] = dp[mask] + 1
    return dp[full]


if __name__ == "__main__":
    prefs = {
        0: [0, 1, 3, 6],
        1: [1, 4, 7],
        2: [2, 4, 7, 5],
        3: [3, 2, 5],
        4: [5, 8],
    }
    print(min_drinks(prefs))  # 2
