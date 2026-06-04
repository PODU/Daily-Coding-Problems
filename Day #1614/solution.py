# Day 1614: Decode ways: DP rolling two states. Add prev1 if current digit valid, prev2 if last two in 10..26.
# Time: O(n), Space: O(1).


def num_decodings(s):
    if not s or s[0] == '0':
        return 0
    prev2, prev1 = 1, 1
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
    print(num_decodings("111"))


if __name__ == "__main__":
    main()
