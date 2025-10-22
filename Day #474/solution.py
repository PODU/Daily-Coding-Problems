# Day 474: Min coins via DP over amounts (optimal for arbitrary denominations).
# Greedy also works for canonical US coins {1,5,10,25}. Time: O(n*k), Space: O(n).


def min_coins(n, coins):
    INF = float("inf")
    dp = [INF] * (n + 1)
    dp[0] = 0
    for a in range(1, n + 1):
        for c in coins:
            if c <= a and dp[a - c] != INF:
                dp[a] = min(dp[a], dp[a - c] + 1)
    return dp[n]


def main():
    coins = [1, 5, 10, 25]
    n = 16
    print(min_coins(n, coins))


if __name__ == "__main__":
    main()
