# Day 1133: Phone keypad letter combinations via backtracking. O(prod of choices) time.

M = {
    '2': "abc", '3': "def", '4': "ghi", '5': "jkl",
    '6': "mno", '7': "pqrs", '8': "tuv", '9': "wxyz",
}


def letter_combinations(digits):
    if not digits:
        return []
    out = []

    def backtrack(i, cur):
        if i == len(digits):
            out.append("".join(cur))
            return
        for c in M[digits[i]]:
            cur.append(c)
            backtrack(i + 1, cur)
            cur.pop()

    backtrack(0, [])
    return out


if __name__ == "__main__":
    res = letter_combinations("23")
    print("[" + ", ".join(res) + "]")
