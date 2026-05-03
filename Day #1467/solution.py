# Day 1467: All permutations via backtracking, picking remaining elements left-to-right (lexicographic order).
# Time O(n! * n), Space O(n) recursion + output.

def permute(nums):
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


if __name__ == "__main__":
    res = permute([1, 2, 3])
    out = "[" + ",".join("[" + ",".join(str(x) for x in p) + "]" for p in res) + "]"
    print(out)
