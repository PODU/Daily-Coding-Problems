# Day 1349: Reconstruct a BST from its postorder traversal.
# Consume postorder right-to-left with value bounds (right subtree before left). O(n) time, O(h) space.
from typing import Optional, List


class Node:
    def __init__(self, val: int):
        self.val = val
        self.left: Optional[Node] = None
        self.right: Optional[Node] = None


def reconstruct(post: List[int]) -> Optional[Node]:
    idx = len(post) - 1

    def build(bound: float) -> Optional[Node]:
        nonlocal idx
        if idx < 0 or post[idx] < bound:
            return None
        node = Node(post[idx])
        idx -= 1
        node.right = build(node.val)
        node.left = build(bound)
        return node

    return build(float("-inf"))


def preorder(n, out):
    if not n:
        return
    out.append(n.val)
    preorder(n.left, out)
    preorder(n.right, out)


def inorder(n, out):
    if not n:
        return
    inorder(n.left, out)
    out.append(n.val)
    inorder(n.right, out)


if __name__ == "__main__":
    root = reconstruct([2, 4, 3, 8, 7, 5])
    pre, ino = [], []
    preorder(root, pre)
    inorder(root, ino)
    print("Root:", root.val)
    print("Preorder:", " ".join(map(str, pre)))
    print("Inorder:", " ".join(map(str, ino)))
