# Day 413: Ordered ways to climb a staircase with allowed step sizes X.
# DP: ways[n] = sum over x in X of ways[n-x]. Time O(N*|X|), Space O(N).
def count_ways(n, steps):
    ways = [0] * (n + 1)
    ways[0] = 1
    for i in range(1, n + 1):
        for x in steps:
            if x <= i:
                ways[i] += ways[i - x]
    return ways[n]


if __name__ == "__main__":
    print(count_ways(4, [1, 2]))        # 5
    print(count_ways(10, [1, 3, 5]))    # generalized X={1,3,5}
