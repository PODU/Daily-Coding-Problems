# Day 1444: Convert a binary tree to a full binary tree by removing every node
# with exactly one child (its single child is promoted up).
# Post-order recursion. Time O(n), Space O(h).
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def to_full(root):
    if root is None:
        return None
    root.left = to_full(root.left)
    root.right = to_full(root.right)
    if root.left and not root.right:
        return root.left
    if root.right and not root.left:
        return root.right
    return root


def preorder(r, out):
    if r is None:
        return
    out.append(r.val)
    preorder(r.left, out)
    preorder(r.right, out)


if __name__ == "__main__":
    n0 = Node(0, Node(1), Node(2))
    n0.left.left = Node(3)
    n0.left.left.right = Node(5)
    n0.right.right = Node(4)
    n0.right.right.left = Node(6)
    n0.right.right.right = Node(7)

    full = to_full(n0)
    out = []
    preorder(full, out)
    print("Preorder of full tree:", " ".join(map(str, out)))  # 0 5 4 6 7
