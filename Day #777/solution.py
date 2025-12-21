# Day 777: Ternary Search Tree with insert and search.
# Each node has left/mid/right children. O(L * log A) per op (L=word length).


class Node:
    __slots__ = ("c", "end", "l", "m", "r")

    def __init__(self, c):
        self.c = c
        self.end = False
        self.l = self.m = self.r = None


class TST:
    def __init__(self):
        self.root = None

    def _insert(self, node, w, i):
        c = w[i]
        if node is None:
            node = Node(c)
        if c < node.c:
            node.l = self._insert(node.l, w, i)
        elif c > node.c:
            node.r = self._insert(node.r, w, i)
        elif i + 1 < len(w):
            node.m = self._insert(node.m, w, i + 1)
        else:
            node.end = True
        return node

    def insert(self, w):
        if w:
            self.root = self._insert(self.root, w, 0)

    def search(self, w):
        node, i = self.root, 0
        while node and w:
            c = w[i]
            if c < node.c:
                node = node.l
            elif c > node.c:
                node = node.r
            elif i + 1 == len(w):
                return node.end
            else:
                node = node.m
                i += 1
        return False


if __name__ == "__main__":
    t = TST()
    for w in ["code", "cob", "be", "ax", "war", "we"]:
        t.insert(w)
    print(t.search("cob"), t.search("code"), t.search("cod"), t.search("we"))
    # True True False True
