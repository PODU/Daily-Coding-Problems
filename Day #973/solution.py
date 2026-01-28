# Day 973: Count ways to decode a digit string (a=1..z=26).
# Approach: DP, dp[i]=dp[i-1] if 1-digit valid + dp[i-2] if 2-digit valid. Time O(n), Space O(1).


def num_decodings(s):
    if not s or s[0] == '0':
        return 0
    prev2, prev1 = 1, 1
    for i in range(1, len(s)):
        cur = 0
        if s[i] != '0':
            cur += prev1
        two = int(s[i - 1:i + 1])
        if 10 <= two <= 26:
            cur += prev2
        prev2, prev1 = prev1, cur
    return prev1


if __name__ == '__main__':
    print(num_decodings("111"))  # 3
