# Day 1402: Huffman algorithm repeatedly merges the two lowest-frequency nodes via a
# min-heap to build an optimal prefix tree, then DFS assigns codes (left=0,right=1).
# Time O(C log C) for C distinct chars, Space O(C).
import heapq


class Node:
    def __init__(self, freq, ch=None, left=None, right=None):
        self.freq, self.ch, self.left, self.right = freq, ch, left, right


def build_codes(freq):
    heap = []
    for i, (ch, f) in enumerate(freq.items()):
        heapq.heappush(heap, (f, i, Node(f, ch)))
    cnt = len(freq)
    while len(heap) > 1:
        f1, _, n1 = heapq.heappop(heap)
        f2, _, n2 = heapq.heappop(heap)
        heapq.heappush(heap, (f1 + f2, cnt, Node(f1 + f2, None, n1, n2)))
        cnt += 1
    root = heap[0][2]
    codes = {}

    def dfs(node, path):
        if node.ch is not None:
            codes[node.ch] = path or "0"
            return
        dfs(node.left, path + "0")
        dfs(node.right, path + "1")

    dfs(root, "")
    return codes


if __name__ == "__main__":
    freq = {'c': 1, 'a': 4, 't': 3, 's': 2}
    codes = build_codes(freq)
    for ch in sorted(codes):
        print(f"{ch}: {codes[ch]}")
    word = "cats"
    print("encode(cats):", "".join(codes[c] for c in word))
