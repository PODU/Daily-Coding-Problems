# Day 810: Boustrophedon (zigzag) level-order traversal of a binary tree.
# BFS level by level, reversing every other level. Time O(N), Space O(N).
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def boustrophedon(root):
    out = []
    if not root:
        return out
    q = deque([root])
    ltr = True
    while q:
        level = []
        for _ in range(len(q)):
            n = q.popleft()
            level.append(n.val)
            if n.left:
                q.append(n.left)
            if n.right:
                q.append(n.right)
        out.extend(level if ltr else level[::-1])
        ltr = not ltr
    return out


if __name__ == "__main__":
    root = Node(1)
    root.left, root.right = Node(2), Node(3)
    root.left.left, root.left.right = Node(4), Node(5)
    root.right.left, root.right.right = Node(6), Node(7)
    print(boustrophedon(root))  # [1, 3, 2, 4, 5, 6, 7]
