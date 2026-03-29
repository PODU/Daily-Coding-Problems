# Day 1275: Longest palindromic substring via Manacher's algorithm. O(n) time, O(n) space.

def longest_palindrome(s: str) -> str:
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
    max_len = max(p)
    center_index = p.index(max_len)
    start = (center_index - max_len) // 2
    return s[start:start + max_len]


if __name__ == "__main__":
    print(f'"{longest_palindrome("aabcdcb")}"')  # "bcdcb"
    print(f'"{longest_palindrome("bananas")}"')  # "anana"
