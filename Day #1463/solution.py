# Day 1463: Pythagorean triplet: square all, sort, for each largest c^2 two-pointer for a^2+b^2.
# Time: O(n^2), Space: O(n).
def has_triplet(arr):
    sq = sorted(x * x for x in arr)
    n = len(sq)
    for c in range(n - 1, 1, -1):
        l, r = 0, c - 1
        while l < r:
            s = sq[l] + sq[r]
            if s == sq[c]:
                return True
            elif s < sq[c]:
                l += 1
            else:
                r -= 1
    return False


if __name__ == "__main__":
    print("True" if has_triplet([3, 1, 4, 6, 5]) else "False")
    print("True" if has_triplet([10, 4, 6, 12, 5]) else "False")
