# Day 597: Detect a Pythagorean triplet a^2 + b^2 = c^2 in an array.
# Approach: square values, sort, fix c as the largest, two-pointer. Time O(n^2), Space O(n).


def has_pythagorean_triplet(nums):
    sq = sorted(v * v for v in nums)
    n = len(sq)
    for c in range(n - 1, 1, -1):
        lo, hi = 0, c - 1
        while lo < hi:
            s = sq[lo] + sq[hi]
            if s == sq[c]:
                return True
            elif s < sq[c]:
                lo += 1
            else:
                hi -= 1
    return False


def main():
    arr = [3, 1, 4, 6, 5]   # contains 3,4,5
    print(str(has_pythagorean_triplet(arr)).lower())


if __name__ == '__main__':
    main()
