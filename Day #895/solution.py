# Day 895: Palindrome by deleting at most k chars: min deletions = n - LPS(s).
# LPS via interval DP (space-optimized to O(n)). Time O(n^2), Space O(n).
def can_make_palindrome(s, k):
    n = len(s)
    if n == 0:
        return 0 <= k
    prev = [0] * n
    cur = [0] * n
    for i in range(n - 1, -1, -1):
        cur = [0] * n
        cur[i] = 1
        for j in range(i + 1, n):
            if s[i] == s[j]:
                cur[j] = prev[j - 1] + 2
            else:
                cur[j] = max(prev[j], cur[j - 1])
        prev = cur
    lps = cur[n - 1]
    return (n - lps) <= k


if __name__ == "__main__":
    print("True" if can_make_palindrome("waterrfetawx", 2) else "False")
