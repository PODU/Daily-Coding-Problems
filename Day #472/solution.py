# Day 472: Count decodings (a=1..z=26) with bottom-up DP keeping only two running states.
# Time: O(n), Space: O(1).


def num_decodings(s):
    if not s or s[0] == '0':
        return 0
    prev2 = 1  # ways up to i-2
    prev1 = 1  # ways up to i-1
    for i in range(1, len(s)):
        cur = 0
        if s[i] != '0':
            cur += prev1
        two = int(s[i - 1]) * 10 + int(s[i])
        if 10 <= two <= 26:
            cur += prev2
        prev2, prev1 = prev1, cur
    return prev1


def main():
    msg = "111"
    print(num_decodings(msg))


if __name__ == "__main__":
    main()
