# Day 1325: Sorted array -> height-balanced BST.
# Recursively pick the middle element as the root so both halves differ in height by <=1. O(n) time, O(log n) stack.

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build(a, lo, hi):
    if lo > hi:
        return None
    mid = (lo + hi) // 2
    root = Node(a[mid])
    root.left = build(a, lo, mid - 1)
    root.right = build(a, mid + 1, hi)
    return root


def preorder(node, out):
    if not node:
        return
    out.append(node.val)
    preorder(node.left, out)
    preorder(node.right, out)


if __name__ == "__main__":
    a = [1, 2, 3, 4, 5, 6, 7]
    root = build(a, 0, len(a) - 1)
    out = []
    preorder(root, out)
    print("preorder:", out)  # preorder: [4, 2, 1, 3, 6, 5, 7]
