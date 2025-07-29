# Day 48: Reconstruct binary tree from preorder + inorder.
# Hashmap of inorder positions; recurse. Time: O(n), Space: O(n).
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build_tree(preorder, inorder):
    pos = {v: i for i, v in enumerate(inorder)}
    pre_iter = iter(preorder)

    def helper(in_l, in_r):
        if in_l > in_r:
            return None
        root_val = next(pre_iter)
        root = Node(root_val)
        mid = pos[root_val]
        root.left = helper(in_l, mid - 1)
        root.right = helper(mid + 1, in_r)
        return root

    return helper(0, len(inorder) - 1)


def level_order(root):
    out, q = [], deque([root])
    while q:
        n = q.popleft()
        out.append(n.val)
        if n.left:
            q.append(n.left)
        if n.right:
            q.append(n.right)
    return out


if __name__ == "__main__":
    pre = ["a", "b", "d", "e", "c", "f", "g"]
    ino = ["d", "b", "e", "a", "f", "c", "g"]
    root = build_tree(pre, ino)
    # Level-order traversal confirms reconstruction: a b c d e f g
    print(" ".join(level_order(root)))
