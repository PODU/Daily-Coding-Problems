# Day 1156: Level-order (BFS) traversal of a binary tree using a queue. O(n) time, O(n) space.
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def level_order(root):
    res = []
    if root is None:
        return res
    q = deque([root])
    while q:
        cur = q.popleft()
        res.append(cur.val)
        if cur.left:
            q.append(cur.left)
        if cur.right:
            q.append(cur.right)
    return res


def main():
    root = Node(1)
    root.left = Node(2)
    root.right = Node(3)
    root.right.left = Node(4)
    root.right.right = Node(5)

    vals = level_order(root)
    print(", ".join(str(v) for v in vals))


if __name__ == "__main__":
    main()
