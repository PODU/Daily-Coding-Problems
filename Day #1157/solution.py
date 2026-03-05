# Day 1157: H-index via counting buckets: bucket papers by min(citation, n), scan from high. O(n) time, O(n) space.


def h_index(citations):
    n = len(citations)
    bucket = [0] * (n + 1)
    for c in citations:
        bucket[min(c, n)] += 1
    count = 0
    for h in range(n, -1, -1):
        count += bucket[h]
        if count >= h:
            return h
    return 0


def main():
    citations = [4, 3, 0, 1, 5]
    print(h_index(citations))


if __name__ == "__main__":
    main()
