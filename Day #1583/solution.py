# Day 1583: Bottom view of a binary tree.
# BFS tracking horizontal distance; last node seen per hd (deepest wins). Time: O(n log n); Space: O(n).
from collections import deque


class Node:
    def __init__(self, val, l=None, r=None):
        self.val, self.l, self.r = val, l, r


def bottom_view(root):
    if not root:
        return []
    hd_val = {}
    q = deque([(root, 0)])
    while q:
        n, hd = q.popleft()
        hd_val[hd] = n.val  # overwrite -> deepest wins
        if n.l:
            q.append((n.l, hd - 1))
        if n.r:
            q.append((n.r, hd + 1))
    return [hd_val[k] for k in sorted(hd_val)]


if __name__ == "__main__":
    root = Node(5,
                Node(3, Node(1, Node(0)), Node(4)),
                Node(7, Node(6), Node(9, Node(8))))
    print(bottom_view(root))  # [0, 1, 3, 6, 8, 9]
