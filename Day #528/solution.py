# Day 528: Huffman coding. Build a prefix tree by repeatedly merging the two
# lowest-frequency nodes (min-heap), then read each char's code from its
# root-to-leaf path (left=0, right=1). Time O(n log n), space O(n).
# Note: the README's example tree is illustrative, not a strict Huffman tree;
# this produces a correct, deterministic optimal-prefix Huffman mapping.
import heapq


def build_huffman(freq: dict) -> dict:
    # heap entries: (freq, order, char, left, right); order makes ties deterministic
    order = 0
    heap = []
    for ch, f in freq.items():
        heapq.heappush(heap, (f, order, ch, None, None))
        order += 1
    while len(heap) > 1:
        a = heapq.heappop(heap)
        b = heapq.heappop(heap)
        heapq.heappush(heap, (a[0] + b[0], order, None, a, b))
        order += 1
    root = heap[0]

    codes = {}

    def walk(node, path):
        _, _, ch, left, right = node
        if left is None and right is None:
            codes[ch] = path or "0"
            return
        walk(left, path + "0")
        walk(right, path + "1")

    walk(root, "")
    return codes


if __name__ == "__main__":
    freq = {"c": 1, "a": 1, "t": 1, "s": 1}
    codes = build_huffman(freq)
    for ch in sorted(codes):
        print(f"{ch} -> {codes[ch]}")

    word = "cats"
    encoded = "".join(codes[c] for c in word)
    print(f"{word} -> {encoded}")
