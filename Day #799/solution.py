# Day 799: PrefixMapSum - trie where each node stores sum of values below it.
# insert overwrites via delta (new-old) propagated along the path.
# insert O(L), sum O(L). Space O(total chars).


class Node:
    __slots__ = ("total", "child")

    def __init__(self):
        self.total = 0
        self.child = {}


class PrefixMapSum:
    def __init__(self):
        self.root = Node()
        self.vals = {}

    def insert(self, key, value):
        delta = value - self.vals.get(key, 0)
        self.vals[key] = value
        cur = self.root
        cur.total += delta
        for c in key:
            if c not in cur.child:
                cur.child[c] = Node()
            cur = cur.child[c]
            cur.total += delta

    def sum(self, prefix):
        cur = self.root
        for c in prefix:
            if c not in cur.child:
                return 0
            cur = cur.child[c]
        return cur.total


if __name__ == "__main__":
    m = PrefixMapSum()
    m.insert("columnar", 3)
    print(m.sum("col"))  # 3
    m.insert("column", 2)
    print(m.sum("col"))  # 5
