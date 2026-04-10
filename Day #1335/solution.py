# Day 1335: Count decodings of a digit string with a=1..z=26.
# DP: ways[i] += ways[i-1] if digit valid, += ways[i-2] if two-digit 10..26 valid. O(n) time, O(1) space.

def num_decodings(s):
    n = len(s)
    if n == 0:
        return 0
    prev2, prev1 = 1, (1 if s[0] != "0" else 0)
    for i in range(1, n):
        cur = 0
        if s[i] != "0":
            cur += prev1
        two = int(s[i - 1:i + 1])
        if 10 <= two <= 26:
            cur += prev2
        prev2, prev1 = prev1, cur
    return prev1


if __name__ == "__main__":
    print(num_decodings("111"))  # 3
