# Day 678: Next permutation of digits: find pivot, swap with next-larger suffix digit,
# reverse suffix. Time: O(d) digits, Space: O(d).

def next_permutation(num):
    d = list(str(num))
    n = len(d)
    i = n - 2
    while i >= 0 and d[i] >= d[i + 1]:
        i -= 1
    if i < 0:
        return num  # already largest permutation
    j = n - 1
    while d[j] <= d[i]:
        j -= 1
    d[i], d[j] = d[j], d[i]
    d[i + 1:] = reversed(d[i + 1:])
    return int("".join(d))


def main():
    print(next_permutation(48975))


if __name__ == "__main__":
    main()
