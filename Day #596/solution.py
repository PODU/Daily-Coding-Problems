# Day 596: Invert a binary tree (mirror it).
# Approach: recursively swap left/right children. Time O(n), Space O(h).
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def invert(root):
    if root is None:
        return
    root.left, root.right = root.right, root.left
    invert(root.left)
    invert(root.right)


def main():
    a, b, c = Node('a'), Node('b'), Node('c')
    d, e, f = Node('d'), Node('e'), Node('f')
    a.left, a.right = b, c
    b.left, b.right = d, e
    c.left = f

    invert(a)

    q = deque([a])
    while q:
        line = []
        for _ in range(len(q)):
            n = q.popleft()
            line.append(n.val)
            if n.left:
                q.append(n.left)
            if n.right:
                q.append(n.right)
        print(' '.join(line))


if __name__ == '__main__':
    main()
