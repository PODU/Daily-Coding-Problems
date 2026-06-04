# Day 1611: Orderly Queue: move one of the first k letters to the end repeatedly; find lexicographically smallest.
# k==1 -> smallest rotation; k>=2 -> sorted ascending. Time O(n^2) (k==1) / O(n log n), Space O(n).


def smallest(s, k):
    if k >= 2:
        return "".join(sorted(s))
    # k == 1: smallest rotation
    return min(s[i:] + s[:i] for i in range(len(s)))


def main():
    print(smallest("daily", 1))


if __name__ == "__main__":
    main()
