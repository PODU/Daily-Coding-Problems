# Day 215: Bottom view of a binary tree.
# Approach: BFS tracking horizontal distance; overwrite map[hd] so last (deepest) node wins. Time O(n log n), Space O(n).
from collections import deque
from typing import List, Optional


class Node:
    def __init__(self, val: int, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def bottom_view(root: Optional[Node]) -> List[int]:
    if not root:
        return []
    hd_map = {}
    q = deque([(root, 0)])
    while q:
        node, hd = q.popleft()
        hd_map[hd] = node.val
        if node.left:
            q.append((node.left, hd - 1))
        if node.right:
            q.append((node.right, hd + 1))
    return [hd_map[k] for k in sorted(hd_map)]


if __name__ == "__main__":
    root = Node(5,
                Node(3, Node(1, Node(0)), Node(4)),
                Node(7, Node(6), Node(9, Node(8))))
    print(bottom_view(root))
