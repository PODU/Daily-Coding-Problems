# Day 605: h-index via counting/bucket sort: bucket citations capped at N, scan from high
# to low accumulating paper counts. Time O(n), Space O(n).
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
