# Day 1545: Pyramid min cost (only lowering). For each index the max pyramid height is
# min of a left pass (rise +1 from edge) and a right pass. A pyramid of peak h
# retains h*h stones, so cost = sum(a) - max(h)^2. Time O(n), Space O(n).
def min_cost(a):
    n = len(a)
    left = [0] * n
    right = [0] * n
    left[0] = min(a[0], 1)
    for i in range(1, n):
        left[i] = min(a[i], left[i-1] + 1)
    right[n-1] = min(a[n-1], 1)
    for i in range(n-2, -1, -1):
        right[i] = min(a[i], right[i+1] + 1)
    best_peak = max(min(left[i], right[i]) for i in range(n))
    return sum(a) - best_peak * best_peak


if __name__ == "__main__":
    print(min_cost([1, 1, 3, 3, 2, 1]))
