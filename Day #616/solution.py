# Day 616: Cryptarithmetic solver via backtracking over distinct letters with column-sum check.
# Time: O(10!) worst case over distinct letters (<=10), Space: O(#letters).


def solve_cryptarithm(word1, word2, word3):
    # Preserve first-appearance order of letters for stable output.
    seen = []
    for c in word1 + word2 + word3:
        if c not in seen:
            seen.append(c)
    leading = {word1[0], word2[0], word3[0]}
    assign = {}
    used = [False] * 10

    def value(w):
        v = 0
        for c in w:
            v = v * 10 + assign[c]
        return v

    def backtrack(idx):
        if idx == len(seen):
            return value(word1) + value(word2) == value(word3)
        ch = seen[idx]
        for d in range(10):
            if used[d]:
                continue
            if d == 0 and ch in leading:
                continue
            used[d] = True
            assign[ch] = d
            if backtrack(idx + 1):
                return True
            used[d] = False
        return False

    if backtrack(0):
        return {c: assign[c] for c in seen}
    return None


if __name__ == "__main__":
    result = solve_cryptarithm("SEND", "MORE", "MONEY")
    print(result)
