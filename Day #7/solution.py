# Day 7: Decode ways: DP where ways[i] depends on 1-digit and valid 2-digit suffixes.
# Time: O(n), Space: O(1).
def num_decodings(s):
    if not s or s[0] == "0":
        return 0
    prev2, prev1 = 1, 1
    for i in range(1, len(s)):
        cur = 0
        if s[i] != "0":
            cur += prev1
        two = int(s[i - 1:i + 1])
        if 10 <= two <= 26:
            cur += prev2
        prev2, prev1 = prev1, cur
    return prev1


if __name__ == "__main__":
    print(num_decodings("111"))
