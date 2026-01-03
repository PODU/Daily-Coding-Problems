# Day 842: invert (mirror) a binary tree by swapping children at every node.
# Recursive DFS. O(n) time, O(h) stack space.
from collections import deque


class Node:
    def __init__(self, v, l=None, r=None):
        self.v, self.l, self.r = v, l, r


def invert(root):
    if root is None:
        return None
    root.l, root.r = root.r, root.l
    invert(root.l)
    invert(root.r)
    return root


def level_order(root):
    if root is None:
        return ""
    out, q = [], deque([root])
    while q:
        n = q.popleft()
        out.append(n.v)
        if n.l:
            q.append(n.l)
        if n.r:
            q.append(n.r)
    return " ".join(out)


if __name__ == "__main__":
    a = Node('a'); b = Node('b'); c = Node('c')
    d = Node('d'); e = Node('e'); f = Node('f')
    a.l, a.r = b, c
    b.l, b.r = d, e
    c.l = f
    invert(a)
    print(level_order(a))  # a c b f e d
