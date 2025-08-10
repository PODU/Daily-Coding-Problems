# Day 96: All permutations via backtracking (swap-in-place). Produces n! perms in
# lexicographic order here by recursing on remaining elements. O(n*n!) time.
def permutations(nums):
    res = []

    def backtrack(path, remaining):
        if not remaining:
            res.append(path[:])
            return
        for i in range(len(remaining)):
            backtrack(path + [remaining[i]], remaining[:i] + remaining[i + 1:])

    backtrack([], nums)
    return res


if __name__ == "__main__":
    print(permutations([1, 2, 3]))
    # [[1, 2, 3], [1, 3, 2], [2, 1, 3], [2, 3, 1], [3, 1, 2], [3, 2, 1]]
