# Day 232: PrefixMapSum: Trie where each node stores the running sum of values passing through it.
# On overwrite, propagate delta = new - old. insert/sum both O(key length).


class PrefixMapSum:
    def __init__(self):
        self.root = {}
        self.vals = {}

    def insert(self, key, value):
        delta = value - self.vals.get(key, 0)
        self.vals[key] = value
        node = self.root
        for c in key:
            nxt = node.setdefault(c, {})
            nxt['sum'] = nxt.get('sum', 0) + delta
            node = nxt

    def sum(self, prefix):
        node = self.root
        for c in prefix:
            if c not in node:
                return 0
            node = node[c]
        return node.get('sum', 0)


if __name__ == "__main__":
    mapsum = PrefixMapSum()
    mapsum.insert("columnar", 3)
    print(mapsum.sum("col"))  # 3
    mapsum.insert("column", 2)
    print(mapsum.sum("col"))  # 5
