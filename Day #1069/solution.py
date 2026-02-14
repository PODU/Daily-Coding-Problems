# Day 1069: Coin game DP: dp[i][j] = max guaranteed for current player on coins[i..j]. O(n^2) time/space.

def coin_game(v):
    n = len(v)
    if n == 0:
        return 0
    dp = [[0] * n for _ in range(n)]
    for i in range(n):
        dp[i][i] = v[i]
    for length in range(2, n + 1):
        for i in range(n - length + 1):
            j = i + length - 1
            take_i = v[i] + min(
                dp[i+2][j] if i+2 <= j else 0,
                dp[i+1][j-1] if i+1 <= j-1 else 0
            )
            take_j = v[j] + min(
                dp[i+1][j-1] if i+1 <= j-1 else 0,
                dp[i][j-2] if i <= j-2 else 0
            )
            dp[i][j] = max(take_i, take_j)
    return dp[0][n-1]

if __name__ == "__main__":
    print(f"Max guaranteed: {coin_game([8, 15, 3, 7])}")
    print(f"Max guaranteed: {coin_game([2, 2, 2, 2])}")
