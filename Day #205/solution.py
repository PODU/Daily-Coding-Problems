# Day 205: Next permutation of an integer's digits.
# Standard next-permutation: find rightmost ascent, swap with next-larger suffix digit, reverse suffix.
# Time: O(d), Space: O(d) for digits.


def next_permutation(n):
    s = list(str(n))
    i = len(s) - 2
    while i >= 0 and s[i] >= s[i + 1]:
        i -= 1
    if i < 0:
        return n  # already the largest permutation
    j = len(s) - 1
    while s[j] <= s[i]:
        j -= 1
    s[i], s[j] = s[j], s[i]
    s[i + 1:] = reversed(s[i + 1:])
    return int("".join(s))


if __name__ == "__main__":
    print(next_permutation(48975))  # 49578
