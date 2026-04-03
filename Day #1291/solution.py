# Day 1291: Next permutation of an integer's digits.
# Standard next-permutation: scan from right, swap, reverse suffix. O(D) time, O(D) space.


def next_permutation(num: str) -> str:
    s = list(num)
    n = len(s)
    i = n - 2
    while i >= 0 and s[i] >= s[i + 1]:
        i -= 1
    if i < 0:
        return num  # already largest
    j = n - 1
    while s[j] <= s[i]:
        j -= 1
    s[i], s[j] = s[j], s[i]
    s[i + 1:] = reversed(s[i + 1:])
    return "".join(s)


if __name__ == "__main__":
    print(next_permutation("48975"))  # 49578
