# Day 1699: Count ways to roll N dice (faces each) summing to total via rolling 1D DP.
# Time O(N*total*faces), Space O(total).
def throw_dice(n, faces, total):
    dp = [0] * (total + 1)
    dp[0] = 1
    for _ in range(n):
        ndp = [0] * (total + 1)
        for s in range(total + 1):
            if not dp[s]:
                continue
            for f in range(1, faces + 1):
                if s + f <= total:
                    ndp[s + f] += dp[s]
        dp = ndp
    return dp[total]


if __name__ == "__main__":
    print(throw_dice(3, 6, 7))
