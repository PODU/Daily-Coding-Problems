# Day 1273: PrefixMapSum - insert(key,value) and sum(prefix).
# Trie storing accumulated sums; insert applies the delta vs the old value.
# insert/sum are O(key length).
from collections import defaultdict


class TrieNode:
    __slots__ = ("sum", "next")

    def __init__(self):
        self.sum = 0
        self.next = {}


class PrefixMapSum:
    def __init__(self):
        self.root = TrieNode()
        self.vals = {}

    def insert(self, key: str, value: int) -> None:
        delta = value - self.vals.get(key, 0)
        self.vals[key] = value
        node = self.root
        for c in key:
            if c not in node.next:
                node.next[c] = TrieNode()
            node = node.next[c]
            node.sum += delta

    def sum(self, prefix: str) -> int:
        node = self.root
        for c in prefix:
            if c not in node.next:
                return 0
            node = node.next[c]
        return node.sum


if __name__ == "__main__":
    mapsum = PrefixMapSum()
    mapsum.insert("columnar", 3)
    print(mapsum.sum("col"))   # 3
    mapsum.insert("column", 2)
    print(mapsum.sum("col"))   # 5
