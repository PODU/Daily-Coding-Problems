# Day 1644: Next permutation of the digit array: find rightmost ascending pair, swap with
# next-greater suffix digit, reverse suffix. Time O(d), Space O(d).

def next_permutation(num: int) -> int:
    s = list(str(num))
    n = len(s)
    i = n - 2
    while i >= 0 and s[i] >= s[i + 1]:
        i -= 1
    if i < 0:
        return -1  # no next permutation
    j = n - 1
    while s[j] <= s[i]:
        j -= 1
    s[i], s[j] = s[j], s[i]
    s[i + 1:] = reversed(s[i + 1:])
    return int(''.join(s))


if __name__ == "__main__":
    print(next_permutation(48975))
