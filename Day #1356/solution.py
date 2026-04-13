# Day 1356: Zigzag (boustrophedon) level order of a binary tree. BFS per level, reverse alternate levels. O(N) time, O(N) space.
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def zigzag(root):
    res = []
    if not root:
        return res
    q = deque([root])
    left_to_right = True
    while q:
        sz = len(q)
        level = [0] * sz
        for i in range(sz):
            cur = q.popleft()
            idx = i if left_to_right else sz - 1 - i
            level[idx] = cur.val
            if cur.left:
                q.append(cur.left)
            if cur.right:
                q.append(cur.right)
        res.extend(level)
        left_to_right = not left_to_right
    return res


if __name__ == "__main__":
    root = Node(1)
    root.left = Node(2)
    root.right = Node(3)
    root.left.left = Node(4)
    root.left.right = Node(5)
    root.right.left = Node(6)
    root.right.right = Node(7)

    print(zigzag(root))
