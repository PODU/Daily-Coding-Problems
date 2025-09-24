# Day 327: Merge two binary trees recursively (sum overlaps, keep lone nodes); print merged tree level-order skipping nulls.
# Time: O(n), Space: O(n).
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
    t1.left = Node(3); t1.right = Node(2)
    t1.left.left = Node(5)

    t2 = Node(2)
    t2.left = Node(1); t2.right = Node(3)
    t2.left.right = Node(4)
    t2.right.right = Node(7)

    m = merge(t1, t2)

    q = deque([m])
    while q:
        line = []
        for _ in range(len(q)):
            cur = q.popleft()
            line.append(str(cur.val))
            if cur.left:
                q.append(cur.left)
            if cur.right:
                q.append(cur.right)
        print(" ".join(line))


if __name__ == "__main__":
    main()
