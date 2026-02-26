# Day 1128: All permutations of a list of digits in lexicographic order.
# Backtracking over sorted digits. O(n!*n) time, O(n) extra space.
def permutations(digits):
    digits = sorted(digits)
    n = len(digits)
    used = [False] * n
    cur = []
    res = []

    def backtrack():
        if len(cur) == n:
            res.append(cur[:])
            return
        for i in range(n):
            if used[i]:
                continue
            used[i] = True
            cur.append(digits[i])
            backtrack()
            cur.pop()
            used[i] = False

    backtrack()
    return res


if __name__ == "__main__":
    res = permutations([1, 2, 3])
    inner = ["[" + ",".join(str(x) for x in p) + "]" for p in res]
    print("[" + ",".join(inner) + "]")
