# Day 1459: Invert binary tree by swapping children recursively; print level-order (BFS).
# Time O(n), Space O(n).
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

    out = []
    q = deque([a])
    while q:
        n = q.popleft()
        if n is None:
            continue
        out.append(n.val)
        q.append(n.left)
        q.append(n.right)
    print(' '.join(out))


if __name__ == '__main__':
    main()
