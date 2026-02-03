# Day 1016: Huffman compression: min-heap repeatedly merges two smallest nodes, then DFS assigns codes (left='0', right='1').
# Tie-break by insertion order for determinism. O(k log k) time, O(k) space.
import heapq


def huffman(freqs):
    heap = []
    counter = 0
    for ch in sorted(freqs):
        heapq.heappush(heap, (freqs[ch], counter, {'ch': ch}))
        counter += 1
    while len(heap) > 1:
        f1, _, n1 = heapq.heappop(heap)
        f2, _, n2 = heapq.heappop(heap)
        heapq.heappush(heap, (f1 + f2, counter, {'l': n1, 'r': n2}))
        counter += 1
    root = heap[0][2]
    codes = {}

    def dfs(node, code):
        if 'ch' in node:
            codes[node['ch']] = code or "0"
            return
        dfs(node['l'], code + "0")
        dfs(node['r'], code + "1")

    dfs(root, "")
    return codes


if __name__ == "__main__":
    freqs = {'a': 5, 'b': 9, 'c': 12, 'd': 13, 'e': 16, 'f': 45}
    codes = huffman(freqs)
    for ch in sorted(codes):
        print(f"{ch}: {codes[ch]}")
