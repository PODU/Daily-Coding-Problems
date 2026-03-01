# Day 1139: LCA via parent pointers: equalize depths then walk up together. O(h) time, O(1) space.
class Node:
    def __init__(self, val):
        self.val = val
        self.parent = None
        self.left = None
        self.right = None


def depth(n):
    d = 0
    while n.parent:
        n = n.parent
        d += 1
    return d


def lca(a, b):
    da, db = depth(a), depth(b)
    while da > db:
        a = a.parent
        da -= 1
    while db > da:
        b = b.parent
        db -= 1
    while a is not b:
        a = a.parent
        b = b.parent
    return a


def link(p, c, left):
    if left:
        p.left = c
    else:
        p.right = c
    c.parent = p
    return c


if __name__ == "__main__":
    n1 = Node(1)
    n2, n3 = Node(2), Node(3)
    n4, n5 = Node(4), Node(5)
    n6, n7 = Node(6), Node(7)
    link(n1, n2, True); link(n1, n3, False)
    link(n2, n4, True); link(n2, n5, False)
    link(n3, n6, True); link(n3, n7, False)

    print(lca(n4, n5).val)
    print(lca(n4, n6).val)
