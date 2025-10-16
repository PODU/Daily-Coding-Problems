# Day 445: Prune a 0/1 binary tree, removing all-zero subtrees.
# Postorder recursion, O(n) time, O(h) space.


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def prune(node):
    if node is None:
        return None
    node.left = prune(node.left)
    node.right = prune(node.right)
    if node.val == 0 and node.left is None and node.right is None:
        return None
    return node


def show(n, prefix="", tag=""):
    if n is None:
        return
    print(prefix + tag + str(n.val))
    show(n.left, prefix + "  ", "L-- ")
    show(n.right, prefix + "  ", "R-- ")


if __name__ == "__main__":
    root = Node(0,
                Node(1),
                Node(0,
                     Node(1, Node(0), Node(0)),
                     Node(0)))
    root = prune(root)
    show(root)
    # 0
    #   L-- 1
    #   R-- 0
    #     L-- 1
