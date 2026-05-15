# Day 1518: Huffman tree built via min-heap, merge two smallest nodes, assign 0/1 edges to derive codes.
# Time: O(n log n) for n symbols. Space: O(n).
import heapq


class Node:
    def __init__(self, ch, freq, order):
        self.ch, self.freq, self.order = ch, freq, order
        self.l = self.r = None


def build(node, code, out):
    if node is None:
        return
    if node.l is None and node.r is None:
        out[node.ch] = code or "0"
        return
    build(node.l, code + "0", out)
    build(node.r, code + "1", out)


def main():
    freqs = {'a': 5, 'b': 9, 'c': 12, 'd': 13, 'e': 16, 'f': 45}
    heap = []
    counter = 0
    for ch, f in freqs.items():
        heapq.heappush(heap, (f, counter, Node(ch, f, counter)))
        counter += 1
    while len(heap) > 1:
        fa, _, a = heapq.heappop(heap)
        fb, _, b = heapq.heappop(heap)
        m = Node('\0', fa + fb, counter)
        m.l, m.r = a, b
        heapq.heappush(heap, (fa + fb, counter, m))
        counter += 1
    root = heap[0][2]
    codes = {}
    build(root, "", codes)
    total_bits = 0
    for ch in sorted(codes):
        print(f"{ch}: {codes[ch]}")
        total_bits += len(codes[ch]) * freqs[ch]
    print(f"Total encoded bits: {total_bits}")


if __name__ == "__main__":
    main()
