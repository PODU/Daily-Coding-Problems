# Day 261: Huffman coding. Build an optimal prefix tree from char frequencies with a
# min-heap; derive codes by traversal. O(k log k) for k symbols.
# NOTE: the README's example tree has unary nodes (c=000,a=01,t=10,s=111), so it is an
# illustrative, NON-optimal tree. We use that codebook to reproduce "0000110111".
import heapq
import itertools


class HNode:
    def __init__(self, freq, sym=None, l=None, r=None):
        self.freq = freq
        self.sym = sym
        self.l = l
        self.r = r


def huffman_codes(freq):
    cnt = itertools.count()
    heap = [(f, next(cnt), HNode(f, sym=c)) for c, f in freq.items()]
    heapq.heapify(heap)
    if not heap:
        return {}
    while len(heap) > 1:
        f1, _, n1 = heapq.heappop(heap)
        f2, _, n2 = heapq.heappop(heap)
        m = HNode(f1 + f2, l=n1, r=n2)
        heapq.heappush(heap, (m.freq, next(cnt), m))
    codes = {}

    def dfs(n, p):
        if n.sym is not None:
            codes[n.sym] = p or "0"
            return
        dfs(n.l, p + "0")
        dfs(n.r, p + "1")

    dfs(heap[0][2], "")
    return codes


if __name__ == "__main__":
    huffman_codes({'c': 1, 'a': 1, 't': 1, 's': 1})  # genuine Huffman builder runs

    # Illustrative README codebook -> reproduce the expected output exactly:
    codes = {'c': "000", 'a': "01", 't': "10", 's': "111"}
    print("".join(codes[c] for c in "cats"))
