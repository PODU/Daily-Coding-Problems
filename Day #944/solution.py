# Day 944: Next greater permutation of an integer's digits (in-place on digit array).
# Find pivot, swap with next larger from the right, reverse suffix. Time O(d), Space O(d).


def next_permutation(num):
    d = list(str(num))
    n = len(d)
    i = n - 2
    while i >= 0 and d[i] >= d[i + 1]:
        i -= 1
    if i < 0:
        return -1  # already the largest arrangement
    j = n - 1
    while d[j] <= d[i]:
        j -= 1
    d[i], d[j] = d[j], d[i]
    d[i + 1:] = reversed(d[i + 1:])
    return int("".join(d))


if __name__ == "__main__":
    print(next_permutation(48975))  # 49578
