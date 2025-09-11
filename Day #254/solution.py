# Day 254: Prune to full binary tree: post-order DFS; a node with exactly one child is
# replaced by that child. Returns new root. Time O(n), Space O(h) recursion.

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def prune(root):
    if root is None:
        return None
    root.left = prune(root.left)
    root.right = prune(root.right)
    if root.left and not root.right:
        return root.left
    if root.right and not root.left:
        return root.right
    return root


def preorder(node, out):
    if node is None:
        return
    out.append(node.val)
    preorder(node.left, out)
    preorder(node.right, out)


if __name__ == "__main__":
    root = Node(0)
    root.left = Node(1)
    root.right = Node(2)
    root.left.left = Node(3)
    root.left.left.right = Node(5)
    root.right.right = Node(4)
    root.right.right.left = Node(6)
    root.right.right.right = Node(7)

    root = prune(root)
    pre = []
    preorder(root, pre)
    print("Preorder after pruning: " + " ".join(map(str, pre)))
