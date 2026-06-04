# Day 1612: Sorted array -> height-balanced BST: pick lower-middle as root, recurse. Print preorder.
# mid = (lo+hi)//2 (lower-middle). Time O(n), Space O(log n) recursion.


class Node:
    __slots__ = ("val", "left", "right")

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
    if node is None:
        return
    out.append(node.val)
    preorder(node.left, out)
    preorder(node.right, out)


def main():
    a = [-10, -3, 0, 5, 9]
    root = build(a, 0, len(a) - 1)
    out = []
    preorder(root, out)
    print(" ".join(str(v) for v in out))


if __name__ == "__main__":
    main()
