# Day 1104: Phone digit -> letters combinations via backtracking.
# Time: O(prod of choices * len). Space: O(len) recursion.
def letter_combos(mapping, digits):
    if not digits:
        return []
    out = []

    def dfs(i, cur):
        if i == len(digits):
            out.append("".join(cur))
            return
        for c in mapping[digits[i]]:
            cur.append(c)
            dfs(i + 1, cur)
            cur.pop()

    dfs(0, [])
    return out


if __name__ == "__main__":
    mapping = {"2": ["a", "b", "c"], "3": ["d", "e", "f"]}
    print(letter_combos(mapping, "23"))
    # ['ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf']
