# Day 1757: Count ordered ways to climb N stairs (steps 1 or 2 -> Fibonacci).
# Generalized to a step set X via DP: ways[i] = sum of ways[i-x]. O(N*|X|) time, O(N) space.


def climb_ways(n, steps):
    ways = [0] * (n + 1)
    ways[0] = 1
    for i in range(1, n + 1):
        for x in steps:
            if i - x >= 0:
                ways[i] += ways[i - x]
    return ways[n]


def main():
    N = 4
    print(climb_ways(N, [1, 2]))  # 5
    print("Generalized X={1,3,5}, N=4:", climb_ways(N, [1, 3, 5]))


if __name__ == "__main__":
    main()
