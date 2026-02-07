# Day 1037: Sorted array -> height-balanced BST: recursively pick middle as root.
# Time: O(n), Space: O(log n) recursion.

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build(a, lo, hi):
    if lo > hi:
        return None
    mid = lo + (hi - lo) // 2
    root = Node(a[mid])
    root.left = build(a, lo, mid - 1)
    root.right = build(a, mid + 1, hi)
    return root


def print_rotated(node, depth):
    if node is None:
        return
    print_rotated(node.right, depth + 1)
    print(" " * (depth * 4) + str(node.val))
    print_rotated(node.left, depth + 1)


def inorder(node, out):
    if node is None:
        return
    inorder(node.left, out)
    out.append(node.val)
    inorder(node.right, out)


def main():
    a = [-10, -3, 0, 5, 9]
    root = build(a, 0, len(a) - 1)
    print("Height-balanced BST (rotated 90 deg):")
    print_rotated(root, 0)
    io = []
    inorder(root, io)
    print("In-order: " + " ".join(str(x) for x in io))


if __name__ == "__main__":
    main()
