# Day 112: LCA with parent pointers - equalize depths, walk up together. O(h).
class Node:
    def __init__(self, val, parent=None):
        self.val = val
        self.parent = parent
        self.l = None
        self.r = None


def depth(n):
    d = 0
    while n:
        n = n.parent
        d += 1
    return d


def lca(a, b):
    da, db = depth(a), depth(b)
    while da > db:
        a, da = a.parent, da - 1
    while db > da:
        b, db = b.parent, db - 1
    while a is not b:
        a, b = a.parent, b.parent
    return a


if __name__ == "__main__":
    root = Node(3)
    root.l = Node(5, root); root.r = Node(1, root)
    root.l.l = Node(6, root.l); root.l.r = Node(2, root.l)
    root.r.l = Node(0, root.r); root.r.r = Node(8, root.r)
    root.l.r.l = Node(7, root.l.r); root.l.r.r = Node(4, root.l.r)
    print(lca(root.l, root.r).val)            # 3
    print(lca(root.l.r.l, root.l.r.r).val)    # 2
