# Day 990: Count ordered ways to climb N steps using step sizes from set X.
# Bottom-up DP: ways[i] = sum over x in X of ways[i-x]. O(N*|X|) time, O(N) space.


def climb_ways(n, X):
    ways = [0] * (n + 1)
    ways[0] = 1
    for i in range(1, n + 1):
        for x in X:
            if i - x >= 0:
                ways[i] += ways[i - x]
    return ways[n]


if __name__ == "__main__":
    print("N=4, X={1,2}:", climb_ways(4, [1, 2]))       # expected 5
    print("N=4, X={1,3,5}:", climb_ways(4, [1, 3, 5]))  # generalized
