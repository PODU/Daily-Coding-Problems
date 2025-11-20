# Day 632: Deduce coin denominations from a "ways to make change" array.
# Approach: reverse coin-change DP. If ways[i] exceeds the count reachable
# with coins found so far, i is itself a denomination.
# Time: O(N * D), Space: O(N).
def find_denominations(ways):
    n = len(ways)
    dp = [0] * n
    dp[0] = 1
    coins = []
    for i in range(1, n):
        if dp[i] < ways[i]:          # i is a new denomination
            coins.append(i)
            for j in range(i, n):
                dp[j] += dp[j - i]
    return coins


if __name__ == "__main__":
    ways = [1, 0, 1, 1, 2]
    coins = find_denominations(ways)
    if len(coins) > 1:
        print(", ".join(map(str, coins[:-1])) + ", and " + str(coins[-1]))
    else:
        print(", ".join(map(str, coins)))
