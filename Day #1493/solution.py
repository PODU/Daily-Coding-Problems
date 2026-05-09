# Day 1493: Detect a Pythagorean triplet a^2+b^2=c^2 in an array.
# Square + sort, then fix largest as c and two-pointer. Time O(n^2), Space O(1).


def has_triplet(nums):
    sq = sorted(x * x for x in nums)
    n = len(sq)
    for c in range(n - 1, 1, -1):
        a, b = 0, c - 1
        while a < b:
            s = sq[a] + sq[b]
            if s == sq[c]:
                return True
            if s < sq[c]:
                a += 1
            else:
                b -= 1
    return False


if __name__ == "__main__":
    nums = [3, 1, 4, 6, 5]
    print(has_triplet(nums))  # expected: True
