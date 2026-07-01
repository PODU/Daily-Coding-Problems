# Day 1747: All permutations of a list of digits.
# Approach: backtracking with a used[] mask, iterating values in order -> lexicographic.
# Time O(n * n!), space O(n) recursion (plus O(n!) for the output).


def permutations(nums):
    res = []
    used = [False] * len(nums)
    cur = []

    def backtrack():
        if len(cur) == len(nums):
            res.append(cur[:])
            return
        for i in range(len(nums)):
            if used[i]:
                continue
            used[i] = True
            cur.append(nums[i])
            backtrack()
            cur.pop()
            used[i] = False

    backtrack()
    return res


def main():
    nums = [1, 2, 3]
    res = permutations(nums)
    # Print in exact nested-list format with no spaces.
    inner = ",".join("[" + ",".join(map(str, p)) + "]" for p in res)
    print("[" + inner + "]")


if __name__ == "__main__":
    main()
