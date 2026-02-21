# Day 1111: Day 1111 - Lazy bartender (minimum set cover)
# Approach: exact set cover via DP over bitmask of covered customers.
# Time: O(D * 2^C), Space: O(2^C), where C = #customers, D = #drinks.

def min_drinks(preferences):
    customers = sorted(preferences.keys())
    cidx = {c: i for i, c in enumerate(customers)}
    n = len(customers)
    full = (1 << n) - 1

    # drink -> bitmask of customers it satisfies
    drink_mask = {}
    for c, drinks in preferences.items():
        for d in drinks:
            drink_mask[d] = drink_mask.get(d, 0) | (1 << cidx[c])

    INF = float('inf')
    dp = [INF] * (1 << n)
    dp[0] = 0
    for state in range(1 << n):
        if dp[state] == INF:
            continue
        for m in drink_mask.values():
            ns = state | m
            if dp[ns] > dp[state] + 1:
                dp[ns] = dp[state] + 1
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
