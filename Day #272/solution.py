# Day 272: throw_dice(N, faces, total) -> number of ways to reach total.
# 1-D DP over rolling sums. Time O(N*total*faces), Space O(total).


def throw_dice(N, faces, total):
    dp = [0] * (total + 1)
    dp[0] = 1
    for _ in range(N):
        ndp = [0] * (total + 1)
        for t in range(total + 1):
            if not dp[t]:
                continue
            for f in range(1, faces + 1):
                if t + f <= total:
                    ndp[t + f] += dp[t]
        dp = ndp
    return dp[total]


if __name__ == "__main__":
    print(throw_dice(3, 6, 7))  # 15
