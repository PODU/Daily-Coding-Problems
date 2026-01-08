# Day 874: Longest palindromic substring via Manacher's algorithm. Time O(n), Space O(n).

def longest_palindrome(s):
    if not s:
        return ""
    t = "^#" + "#".join(s) + "#$"
    n = len(t)
    p = [0] * n
    center = right = 0
    for i in range(1, n - 1):
        if i < right:
            p[i] = min(right - i, p[2 * center - i])
        while t[i + p[i] + 1] == t[i - p[i] - 1]:
            p[i] += 1
        if i + p[i] > right:
            center, right = i, i + p[i]
    max_len = center_index = 0
    for i in range(1, n - 1):
        if p[i] > max_len:
            max_len, center_index = p[i], i
    start = (center_index - max_len) // 2
    return s[start:start + max_len]


if __name__ == "__main__":
    print('"' + longest_palindrome("aabcdcb") + '"')
    print('"' + longest_palindrome("bananas") + '"')
