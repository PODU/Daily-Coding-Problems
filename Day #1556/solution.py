# Day 1556: Merge two binary trees by summing overlapping nodes; recurse and reuse the
# non-null subtree when only one side exists. Time O(n), Space O(h).
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def merge(a, b):
    if a is None:
        return b
    if b is None:
        return a
    n = Node(a.val + b.val)
    n.left = merge(a.left, b.left)
    n.right = merge(a.right, b.right)
    return n


def main():
    t1 = Node(1)
    t1.left = Node(3)
    t1.right = Node(2)
    t1.left.left = Node(5)

    t2 = Node(2)
    t2.left = Node(1)
    t2.right = Node(3)
    t2.left.right = Node(4)
    t2.right.right = Node(7)

    m = merge(t1, t2)

    out = []
    q = deque([m])
    while q:
        cur = q.popleft()
        if cur is not None:
            out.append(str(cur.val))
            q.append(cur.left)
            q.append(cur.right)
        else:
            out.append("null")
    while out and out[-1] == "null":
        out.pop()

    print(" ".join(out))


if __name__ == "__main__":
    main()
