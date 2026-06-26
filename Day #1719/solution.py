# Day 1719: h-index via counting sort: bucket citations capped at N, scan from high to low
# accumulating papers until count >= citation level. Time O(N), Space O(N).

def h_index(citations):
    n = len(citations)
    bucket = [0] * (n + 1)
    for c in citations:
        bucket[min(c, n)] += 1
    acc = 0
    for h in range(n, -1, -1):
        acc += bucket[h]
        if acc >= h:
            return h
    return 0


if __name__ == "__main__":
    citations = [4, 3, 0, 1, 5]
    print(h_index(citations))
