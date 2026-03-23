# Day 1256: Max contiguous subarray sum, empty allowed: Kadane, clamp running sum at 0.
# Time O(n), Space O(1).
def max_sub(a):
    cur = best = 0
    for x in a:
        cur += x
        if cur < 0:
            cur = 0
        best = max(best, cur)
    return best

if __name__ == "__main__":
    print(max_sub([34, -50, 42, 14, -5, 86]))
    print(max_sub([-5, -1, -8, -9]))
