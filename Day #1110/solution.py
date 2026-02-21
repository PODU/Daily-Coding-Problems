# Day 1110: Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
# Square all, sort; for each largest c use two-pointer scan. Time: O(N^2). Space: O(N).
def has_triplet(arr):
    sq = sorted(x * x for x in arr)
    n = len(sq)
    for c in range(n - 1, 1, -1):
        l, r = 0, c - 1
        while l < r:
            s = sq[l] + sq[r]
            if s == sq[c]:
                return True
            if s < sq[c]:
                l += 1
            else:
                r -= 1
    return False


if __name__ == "__main__":
    print(has_triplet([3, 1, 4, 6, 5]))   # True (3,4,5)
    print(has_triplet([10, 4, 6, 12, 5])) # False
