# Day 497: Convert binary tree to FULL binary tree by removing nodes with exactly one child.
# Post-order recursion: a one-child node is replaced by its processed child.
# Time: O(n), Space: O(h) recursion.


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def full_tree(root):
    if root is None:
        return None
    root.left = full_tree(root.left)
    root.right = full_tree(root.right)
    if root.left and not root.right:
        return root.left
    if root.right and not root.left:
        return root.right
    return root


def preorder(root, out):
    if root is None:
        return
    out.append(str(root.val))
    preorder(root.left, out)
    preorder(root.right, out)


def main():
    root = Node(0)
    root.left = Node(1)
    root.right = Node(2)
    root.left.left = Node(3)
    root.left.left.right = Node(5)
    root.right.right = Node(4)
    root.right.right.left = Node(6)
    root.right.right.right = Node(7)

    root = full_tree(root)
    out = []
    preorder(root, out)
    print(" ".join(out))


if __name__ == "__main__":
    main()
