# Day 479: Generate all permutations via backtracking, picking remaining elements in
# index order so output is lexicographic. Time: O(n! * n), Space: O(n) recursion.


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
    print("[" + ",".join("[" + ",".join(map(str, p)) + "]" for p in res) + "]")


if __name__ == "__main__":
    main()
