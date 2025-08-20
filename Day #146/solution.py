# Day 146: Prune subtrees that contain only 0s via post-order recursion. O(n) time, O(h) stack.


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def prune(root):
    if root is None:
        return None
    root.left = prune(root.left)
    root.right = prune(root.right)
    if root.val == 0 and root.left is None and root.right is None:
        return None
    return root


def preorder(r, out):
    if r is None:
        return
    out.append(str(r.val))
    preorder(r.left, out)
    preorder(r.right, out)


if __name__ == "__main__":
    root = Node(0,
                Node(1),
                Node(0,
                     Node(1, Node(0), Node(0)),
                     Node(0)))
    root = prune(root)
    out = []
    preorder(root, out)
    print("preorder:", " ".join(out))  # 0 1 0 1
