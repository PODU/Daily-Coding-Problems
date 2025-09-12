# Day 258: Boustrophedon (zigzag) level-order traversal of a binary tree.
# BFS level by level, reversing the output order on alternate levels.
# Time: O(n), Space: O(n).
from collections import deque


class TreeNode:
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
        sz = len(q)
        level = [0] * sz
        for i in range(sz):
            node = q.popleft()
            idx = i if left_to_right else sz - 1 - i
            level[idx] = node.val
            if node.left:
                q.append(node.left)
            if node.right:
                q.append(node.right)
        out.extend(level)
        left_to_right = not left_to_right
    return out


def main():
    root = TreeNode(1,
                    TreeNode(2, TreeNode(4), TreeNode(5)),
                    TreeNode(3, TreeNode(6), TreeNode(7)))
    print(boustrophedon(root))  # [1, 3, 2, 4, 5, 6, 7]


if __name__ == "__main__":
    main()
