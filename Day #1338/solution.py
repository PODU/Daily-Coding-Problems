# Day 1338: Longest consecutive run via hash set: start only at run heads (x-1 absent), walk up. O(n) time, O(n) space.


def longest_consecutive(nums):
    s = set(nums)
    best = 0
    for x in s:
        if x - 1 in s:
            continue  # not a run head
        cur, length = x, 1
        while cur + 1 in s:
            cur += 1
            length += 1
        best = max(best, length)
    return best


def main():
    nums = [100, 4, 200, 1, 3, 2]
    print(longest_consecutive(nums))


if __name__ == "__main__":
    main()
