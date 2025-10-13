# Day 422: Merge two binary trees recursively (value = sum), O(n) time, O(h) space.
# Then print merged tree by level-order traversal (skipping null children).
from collections import deque


class Node:
    def __init__(self, v, l=None, r=None):
        self.val = v
        self.left = l
        self.right = r


def merge(a, b):
    if not a:
        return b
    if not b:
        return a
    n = Node(a.val + b.val)
    n.left = merge(a.left, b.left)
    n.right = merge(a.right, b.right)
    return n


if __name__ == "__main__":
    t1 = Node(1, Node(3, Node(5)), Node(2))
    t2 = Node(2, Node(1, None, Node(4)), Node(3, None, Node(7)))
    m = merge(t1, t2)

    out, q = [], deque([m])
    while q:
        c = q.popleft()
        out.append(c.val)
        if c.left:
            q.append(c.left)
        if c.right:
            q.append(c.right)
    print(" ".join(map(str, out)))
