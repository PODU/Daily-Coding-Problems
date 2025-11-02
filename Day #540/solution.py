# Day 540: Boustrophedon (zigzag) level-order traversal of a binary tree.
# BFS level by level, reversing every other level. Time O(n), Space O(n).
from collections import deque


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def boustrophedon(root):
    out = []
    if not root:
        return out
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
        out.extend(level if left_to_right else level[::-1])
        left_to_right = not left_to_right
    return out


if __name__ == "__main__":
    root = Node(1,
                Node(2, Node(4), Node(5)),
                Node(3, Node(6), Node(7)))
    print(boustrophedon(root))
