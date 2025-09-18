# Day 296: Sorted array -> height-balanced BST: pick lower-mid as root, recurse halves; print BFS level-order.
# Time O(n), Space O(log n) recursion.
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.l = None
        self.r = None


def build(a, lo, hi):
    if lo > hi:
        return None
    mid = (lo + hi) // 2
    n = Node(a[mid])
    n.l = build(a, lo, mid - 1)
    n.r = build(a, mid + 1, hi)
    return n


if __name__ == "__main__":
    a = [-10, -3, 0, 5, 9]
    root = build(a, 0, len(a) - 1)
    order = []
    q = deque([root])
    while q:
        n = q.popleft()
        if n is None:
            continue
        order.append(n.val)
        q.append(n.l)
        q.append(n.r)
    print(order)
