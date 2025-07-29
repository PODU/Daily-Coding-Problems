# Day 46: Longest palindromic substring via Manacher's algorithm.
# Time: O(n), Space: O(n).


def longest_palindrome(s: str) -> str:
    if not s:
        return ""
    t = "^#" + "#".join(s) + "#$"
    n = len(t)
    p = [0] * n
    c = r = 0
    for i in range(1, n - 1):
        if i < r:
            p[i] = min(r - i, p[2 * c - i])
        while t[i + 1 + p[i]] == t[i - 1 - p[i]]:
            p[i] += 1
        if i + p[i] > r:
            c, r = i, i + p[i]
    max_len = max(p)
    center = p.index(max_len)
    start = (center - max_len) // 2
    return s[start:start + max_len]


if __name__ == "__main__":
    print('"' + longest_palindrome("aabcdcb") + '"')
    print('"' + longest_palindrome("bananas") + '"')
