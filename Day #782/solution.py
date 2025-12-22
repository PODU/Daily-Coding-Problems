# Day 782: H-index via bucket counting.
# Bucket papers by citation count (capped at N), scan from high to low accumulating. O(n) time, O(n) space.

def h_index(citations):
    n = len(citations)
    bucket = [0] * (n + 1)
    for c in citations:
        bucket[min(c, n)] += 1
    total = 0
    for h in range(n, -1, -1):
        total += bucket[h]
        if total >= h:
            return h
    return 0


if __name__ == "__main__":
    citations = [4, 3, 0, 1, 5]
    print(h_index(citations))
