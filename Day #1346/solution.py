# Day 1346: Expand outward from index i, returning nearest j (by |j-i|) with a[j] > a[i]; None if none.
# Time: O(n) per query, Space: O(1).


def nearest_larger(a, i):
    n = len(a)
    for d in range(1, n):
        l, r = i - d, i + d
        if l >= 0 and a[l] > a[i]:
            return l
        if r < n and a[r] > a[i]:
            return r
    return None


def main():
    a = [4, 1, 3, 5, 6]
    print(nearest_larger(a, 0))


if __name__ == "__main__":
    main()
