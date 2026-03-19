# Day 1236: Count ways to roll N dice (each `faces` sides) summing to `total`.
# DP over dice: dp[s] = #ways. Time O(N*faces*total), Space O(total).


def throw_dice(N, faces, total):
    dp = [0] * (total + 1)
    dp[0] = 1
    for _ in range(N):
        nxt = [0] * (total + 1)
        for s in range(total + 1):
            if dp[s]:
                for f in range(1, faces + 1):
                    if s + f <= total:
                        nxt[s + f] += dp[s]
        dp = nxt
    return dp[total]


if __name__ == "__main__":
    print(throw_dice(3, 6, 7))
