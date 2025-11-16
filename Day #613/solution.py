# Day 613: PrefixMapSum - insert(key,value) and sum(prefix).
# Approach: trie where each node stores total of values passing through; insert propagates delta. Time O(|key|).


class Node:
    __slots__ = ('sum', 'ch')

    def __init__(self):
        self.sum = 0
        self.ch = {}


class PrefixMapSum:
    def __init__(self):
        self.root = Node()
        self.values = {}

    def insert(self, key, value):
        delta = value - self.values.get(key, 0)
        self.values[key] = value
        node = self.root
        for c in key:
            if c not in node.ch:
                node.ch[c] = Node()
            node = node.ch[c]
            node.sum += delta

    def sum(self, prefix):
        node = self.root
        for c in prefix:
            if c not in node.ch:
                return 0
            node = node.ch[c]
        return node.sum


def main():
    m = PrefixMapSum()
    m.insert("columnar", 3)
    print(m.sum("col"))  # 3
    m.insert("column", 2)
    print(m.sum("col"))  # 5


if __name__ == '__main__':
    main()
