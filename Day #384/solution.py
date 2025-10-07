# Day 384: Min coins via bottom-up DP. Returns None if unreachable.
# Time: O(amount * |coins|), Space: O(amount).

def min_coins(coins, amount):
    INF = float("inf")
    dp = [INF] * (amount + 1)
    dp[0] = 0
    for a in range(1, amount + 1):
        for c in coins:
            if c <= a and dp[a - c] + 1 < dp[a]:
                dp[a] = dp[a - c] + 1
    return None if dp[amount] == INF else dp[amount]


if __name__ == "__main__":
    print(min_coins([1, 5, 10], 56))
    print(min_coins([5, 8], 15))
