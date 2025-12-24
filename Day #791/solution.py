# Day 791: throw_dice: DP over dice, dp[s] = ways to reach sum s. Rolling array.
# Time O(N*total*faces), Space O(total).
def throw_dice(N, faces, total):
    dp = [0] * (total + 1)
    dp[0] = 1
    for _ in range(N):
        ndp = [0] * (total + 1)
        for s in range(total + 1):
            if dp[s] == 0:
                continue
            for f in range(1, faces + 1):
                if s + f <= total:
                    ndp[s + f] += dp[s]
        dp = ndp
    return dp[total]


if __name__ == "__main__":
    print(throw_dice(3, 6, 7))
