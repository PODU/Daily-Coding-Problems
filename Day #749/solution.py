# Day 749: Longest palindromic substring via Manacher's algorithm.
# Transform with '#' separators, expand radii using mirror symmetry.
# Time: O(n), Space: O(n).

def longest_palindrome(s):
    if not s:
        return ""
    t = '#' + '#'.join(s) + '#'
    n = len(t)
    p = [0] * n
    c = r = 0
    for i in range(n):
        if i < r:
            p[i] = min(r - i, p[2 * c - i])
        while i - p[i] - 1 >= 0 and i + p[i] + 1 < n and t[i - p[i] - 1] == t[i + p[i] + 1]:
            p[i] += 1
        if i + p[i] > r:
            c, r = i, i + p[i]
    max_len = max(p)
    center = p.index(max_len)
    start = (center - max_len) // 2
    return s[start:start + max_len]


if __name__ == "__main__":
    print(longest_palindrome("aabcdcb"))  # bcdcb
    print(longest_palindrome("bananas"))  # anana
