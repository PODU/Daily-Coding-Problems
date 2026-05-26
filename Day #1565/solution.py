# Day 1565: Subset-sum DP: partition into equal halves iff target=sum/2 reachable. Time O(n*sum), Space O(sum).


def can_partition(nums):
    total = sum(nums)
    if total % 2:
        return False
    target = total // 2
    dp = [False] * (target + 1)
    dp[0] = True
    for x in nums:
        for s in range(target, x - 1, -1):
            if dp[s - x]:
                dp[s] = True
    return dp[target]


def main():
    nums = [15, 5, 20, 10, 35, 15, 10]
    print("true" if can_partition(nums) else "false")


if __name__ == "__main__":
    main()
