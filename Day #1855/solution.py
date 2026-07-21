# Day 1855: LCA in a binary tree with parent pointers.
# Two-pointer walk (like linked-list intersection): swap to the other node at root; meet at LCA. O(h) time, O(1) space.


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None
        self.parent = None


def lca(p, q):
    a, b = p, q
    while a is not b:
        a = q if a is None else a.parent
        b = p if b is None else b.parent
    return a


def attach(parent, child):
    if child:
        child.parent = parent
    return child


if __name__ == "__main__":
    n = {i: Node(i) for i in range(1, 9)}
    n[1].left = attach(n[1], n[2]); n[1].right = attach(n[1], n[3])
    n[2].left = attach(n[2], n[4]); n[2].right = attach(n[2], n[5])
    n[3].right = attach(n[3], n[6])
    n[5].left = attach(n[5], n[7]); n[5].right = attach(n[5], n[8])

    print(lca(n[7], n[8]).val)  # 5
    print(lca(n[7], n[6]).val)  # 1
    print(lca(n[4], n[8]).val)  # 2
