# Day 1036: Reconstruct BST from postorder traversal.
# Approach: walk postorder in reverse (root,right,left) using value bounds.
# Time: O(n), Space: O(h) recursion.
import sys


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def reconstruct(post):
    idx = [len(post) - 1]

    def build(bound):
        if idx[0] < 0 or post[idx[0]] < bound:
            return None
        node = Node(post[idx[0]])
        idx[0] -= 1
        node.right = build(node.val)
        node.left = build(bound)
        return node

    return build(float("-inf"))


def print_sideways(n, depth=0):
    if n is None:
        return
    print_sideways(n.right, depth + 1)
    print("    " * depth + str(n.val))
    print_sideways(n.left, depth + 1)


if __name__ == "__main__":
    post = [2, 4, 3, 8, 7, 5]
    root = reconstruct(post)
    print("Reconstructed BST (rotated 90 deg, root=5):")
    print_sideways(root)
