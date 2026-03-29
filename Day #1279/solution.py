# Day 1279: Lazy bartender = minimum set cover over customers.
# DP over customer bitmask. Time O(2^C * D), Space O(2^C). C=#customers, D=#drinks.
def min_drinks(preferences):
    customers = list(preferences.keys())
    C = len(customers)
    drink_mask = {}
    for i, cust in enumerate(customers):
        for d in preferences[cust]:
            drink_mask[d] = drink_mask.get(d, 0) | (1 << i)
    full = (1 << C) - 1
    INF = float("inf")
    dp = [INF] * (1 << C)
    dp[0] = 0
    masks = list(drink_mask.values())
    for mask in range(full + 1):
        if dp[mask] == INF:
            continue
        for dm in masks:
            nm = mask | dm
            if dp[mask] + 1 < dp[nm]:
                dp[nm] = dp[mask] + 1
    return dp[full]


if __name__ == "__main__":
    preferences = {
        0: [0, 1, 3, 6],
        1: [1, 4, 7],
        2: [2, 4, 7, 5],
        3: [3, 2, 5],
        4: [5, 8],
    }
    print(min_drinks(preferences))  # 2
