# Day 375: h-index via counting sort.
# Bucket citations (capped at n), then scan h from n down accumulating papers
# with >= h citations; first h with count >= h wins. Time O(n), Space O(n).


def h_index(cites):
    n = len(cites)
    buckets = [0] * (n + 1)
    for c in cites:
        buckets[min(c, n)] += 1
    total = 0
    for h in range(n, -1, -1):
        total += buckets[h]
        if total >= h:
            return h
    return 0


if __name__ == "__main__":
    print(h_index([4, 0, 0, 2, 3]))  # 2
