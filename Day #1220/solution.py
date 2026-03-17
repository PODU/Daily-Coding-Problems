# Day 1220: Merge two binary trees recursively (sum overlaps, keep lone nodes). O(min(n1,n2)) time.
# Serialize merged tree as BFS level-order with trailing nulls trimmed.
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def merge(a, b):
    if a is None:
        return b
    if b is None:
        return a
    n = Node(a.val + b.val)
    n.left = merge(a.left, b.left)
    n.right = merge(a.right, b.right)
    return n


def serialize(root):
    out = []
    q = deque([root])
    while q:
        n = q.popleft()
        if n is None:
            out.append("null")
            continue
        out.append(str(n.val))
        q.append(n.left)
        q.append(n.right)
    while out and out[-1] == "null":
        out.pop()
    return "[" + ", ".join(out) + "]"


if __name__ == "__main__":
    t1 = Node(1, Node(3, Node(5)), Node(2))
    t2 = Node(2, Node(1, None, Node(4)), Node(3, None, Node(7)))
    print(serialize(merge(t1, t2)))
