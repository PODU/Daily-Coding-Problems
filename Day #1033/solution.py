# Day 1033: Minimum subset-sum difference (partition into two subsets).
# Boolean subset-sum DP over reachable sums up to total//2; answer total-2*best. O(n*sum) time, O(sum) space.


def min_diff(a):
    total = sum(a)
    dp = [False] * (total // 2 + 1)
    dp[0] = True
    for x in a:
        for s in range(total // 2, x - 1, -1):
            if dp[s - x]:
                dp[s] = True
    for s in range(total // 2, -1, -1):
        if dp[s]:
            return total - 2 * s
    return total


if __name__ == "__main__":
    print(min_diff([5, 10, 15, 20, 25]))
