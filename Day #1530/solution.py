# Day 1530: Zigzag (boustrophedon) level-order traversal: alternate direction each level.
# BFS level by level, reverse odd levels. O(n) time, O(n) space.
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def zigzag(root):
    res = []
    if root is None:
        return res
    q = deque([root])
    left_to_right = True
    while q:
        level = []
        for _ in range(len(q)):
            n = q.popleft()
            level.append(n.val)
            if n.left:
                q.append(n.left)
            if n.right:
                q.append(n.right)
        if not left_to_right:
            level.reverse()
        res.extend(level)
        left_to_right = not left_to_right
    return res


if __name__ == "__main__":
    root = Node(1,
                Node(2, Node(4), Node(5)),
                Node(3, Node(6), Node(7)))
    print(zigzag(root))  # [1, 3, 2, 4, 5, 6, 7]
