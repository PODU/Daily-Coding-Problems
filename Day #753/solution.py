# Day 753: Ternary Search Tree with insert and search.
# Insert/Search: O(L + log n) average, O(L * n) worst; L = key length.


class TSTNode:
    __slots__ = ("c", "is_end", "left", "mid", "right")

    def __init__(self, c):
        self.c = c
        self.is_end = False
        self.left = self.mid = self.right = None


class TST:
    def __init__(self):
        self.root = None

    def _insert(self, node, s, i):
        c = s[i]
        if node is None:
            node = TSTNode(c)
        if c < node.c:
            node.left = self._insert(node.left, s, i)
        elif c > node.c:
            node.right = self._insert(node.right, s, i)
        elif i + 1 < len(s):
            node.mid = self._insert(node.mid, s, i + 1)
        else:
            node.is_end = True
        return node

    def insert(self, s):
        if s:
            self.root = self._insert(self.root, s, 0)

    def search(self, s):
        node, i = self.root, 0
        while node:
            c = s[i]
            if c < node.c:
                node = node.left
            elif c > node.c:
                node = node.right
            else:
                if i + 1 == len(s):
                    return node.is_end
                node = node.mid
                i += 1
        return False


if __name__ == "__main__":
    tst = TST()
    for w in ["code", "cob", "be", "ax", "war", "we"]:
        tst.insert(w)
    for q in ["code", "cob", "cod", "ax", "hello"]:
        print(f"{q}: {str(tst.search(q)).lower()}")
