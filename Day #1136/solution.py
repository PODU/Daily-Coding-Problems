# Day 1136: Bottom view via BFS tracking horizontal distance; last (deepest) node per hd wins. O(n log n).
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def bottom_view(root):
    hd_map = {}
    q = deque([(root, 0)])
    while q:
        node, hd = q.popleft()
        hd_map[hd] = node.val
        if node.left:
            q.append((node.left, hd - 1))
        if node.right:
            q.append((node.right, hd + 1))
    return [hd_map[h] for h in sorted(hd_map)]


if __name__ == "__main__":
    root = Node(5,
                Node(3, Node(1, Node(0)), Node(4)),
                Node(7, Node(6), Node(9, Node(8))))
    print(bottom_view(root))
