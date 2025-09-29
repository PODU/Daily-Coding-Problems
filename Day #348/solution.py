# Day 348: Ternary Search Tree with insert/search. Each node: char + left/mid/right + end flag.
# Time: O(L * log A) per op, Space: O(total chars).
class Node:
    __slots__ = ("c", "end", "left", "mid", "right")
    def __init__(self, c):
        self.c = c
        self.end = False
        self.left = self.mid = self.right = None


class TST:
    def __init__(self):
        self.root = None

    def _insert(self, node, w, i):
        ch = w[i]
        if node is None:
            node = Node(ch)
        if ch < node.c:
            node.left = self._insert(node.left, w, i)
        elif ch > node.c:
            node.right = self._insert(node.right, w, i)
        elif i + 1 < len(w):
            node.mid = self._insert(node.mid, w, i + 1)
        else:
            node.end = True
        return node

    def insert(self, w):
        if w:
            self.root = self._insert(self.root, w, 0)

    def search(self, w):
        node, i = self.root, 0
        while node:
            ch = w[i]
            if ch < node.c:
                node = node.left
            elif ch > node.c:
                node = node.right
            else:
                if i + 1 == len(w):
                    return node.end
                i += 1
                node = node.mid
        return False


if __name__ == "__main__":
    tst = TST()
    for w in ["code", "cob", "be", "ax", "war", "we"]:
        tst.insert(w)
    for q in ["code", "cob", "ax", "c", "war", "cat"]:
        print(f"{q} -> {'true' if tst.search(q) else 'false'}")
