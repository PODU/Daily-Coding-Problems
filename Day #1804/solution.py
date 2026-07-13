# Day 1804: Find a duplicate in array of n+1 elements from {1..n} using a seen set.
# O(n) time, O(n) space.


def find_duplicate(nums):
    seen = set()
    for x in nums:
        if x in seen:
            return x
        seen.add(x)
    return -1  # no duplicate (won't happen per problem constraints)


def main():
    nums = [1, 3, 4, 2, 2]
    print(find_duplicate(nums))  # expected 2


if __name__ == "__main__":
    main()
