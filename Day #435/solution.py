# Day 435: Reconstruct a binary tree from preorder + inorder traversals.
# Approach: recurse, using a dict of inorder value->index to find roots in O(1).
# Time: O(n), Space: O(n).
from collections import deque


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def reconstruct(preorder, inorder):
    idx = {v: i for i, v in enumerate(inorder)}
    state = {"pi": 0}

    def build(in_l, in_r):
        if in_l > in_r:
            return None
        root_val = preorder[state["pi"]]
        state["pi"] += 1
        root = Node(root_val)
        mid = idx[root_val]
        root.left = build(in_l, mid - 1)
        root.right = build(mid + 1, in_r)
        return root

    return build(0, len(inorder) - 1)


if __name__ == "__main__":
    preorder = ["a", "b", "d", "e", "c", "f", "g"]
    inorder = ["d", "b", "e", "a", "f", "c", "g"]
    root = reconstruct(preorder, inorder)

    # Print level-order: a b c d e f g
    out, q = [], deque([root])
    while q:
        n = q.popleft()
        out.append(n.val)
        if n.left:
            q.append(n.left)
        if n.right:
            q.append(n.right)
    print(" ".join(out))
