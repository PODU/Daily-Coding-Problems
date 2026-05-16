# Day 1521: Find all anagram start indices of W in S.
# Sliding window + 26-letter freq + match counter. Time O(|S|), Space O(1).
def find_anagrams(W, S):
    n, m = len(S), len(W)
    res = []
    if m == 0 or m > n:
        return res
    need = [0] * 26
    win = [0] * 26
    for c in W:
        need[ord(c) - 97] += 1
    matches = sum(1 for i in range(26) if need[i] == 0)
    for i in range(n):
        add = ord(S[i]) - 97
        win[add] += 1
        if win[add] == need[add]:
            matches += 1
        elif win[add] == need[add] + 1:
            matches -= 1
        if i >= m:
            rem = ord(S[i - m]) - 97
            win[rem] -= 1
            if win[rem] == need[rem]:
                matches += 1
            elif win[rem] == need[rem] - 1:
                matches -= 1
        if i >= m - 1 and matches == 26:
            res.append(i - m + 1)
    return res


if __name__ == "__main__":
    idx = find_anagrams("ab", "abxaba")
    print(", ".join(map(str, idx)))
