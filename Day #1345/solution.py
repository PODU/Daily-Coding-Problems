# Day 1345: Find smallest window to sort: right bound = last i where a[i] < running max; left bound = first j where a[j] > running min from right.
# Time: O(n), Space: O(1).


def find_unsorted(a):
    n = len(a)
    end, mx = -1, float("-inf")
    for i in range(n):
        if a[i] < mx:
            end = i
        else:
            mx = a[i]
    start, mn = -1, float("inf")
    for i in range(n - 1, -1, -1):
        if a[i] > mn:
            start = i
        else:
            mn = a[i]
    return start, end


def main():
    a = [3, 7, 5, 6, 9]
    start, end = find_unsorted(a)
    print(f"({start}, {end})")


if __name__ == "__main__":
    main()
