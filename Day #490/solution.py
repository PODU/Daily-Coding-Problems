# Day 490: Bottom view of a binary tree.
# BFS by horizontal distance (root 0, left hd-1, right hd+1); the last node seen in BFS
# order for each hd is the lowest. Output sorted by hd. Time O(n log n), Space O(n).
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def bottom_view(root):
    if not root:
        return []
    hd_to_val = {}
    q = deque([(root, 0)])
    while q:
        node, hd = q.popleft()
        hd_to_val[hd] = node.val  # last in BFS order = lowest
        if node.left:
            q.append((node.left, hd - 1))
        if node.right:
            q.append((node.right, hd + 1))
    return [hd_to_val[hd] for hd in sorted(hd_to_val)]


if __name__ == "__main__":
    root = Node(5,
                Node(3, Node(1, Node(0)), Node(4)),
                Node(7, Node(6), Node(9, Node(8))))
    print(bottom_view(root))  # [0, 1, 3, 6, 8, 9]
