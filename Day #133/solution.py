# Day 133: Inorder successor in a BST using parent pointers.
# If right subtree exists, leftmost of it; else climb until node is a left child. O(h) time.


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None
        self.parent = None


def successor(node):
    if not node:
        return None
    if node.right:
        c = node.right
        while c.left:
            c = c.left
        return c
    p = node.parent
    while p and node is p.right:
        node = p
        p = p.parent
    return p


def attach(parent, child):
    if child:
        child.parent = parent
    return child


if __name__ == "__main__":
    root = Node(10)
    root.left = attach(root, Node(5))
    root.right = attach(root, Node(30))
    n22, n35 = Node(22), Node(35)
    root.right.left = attach(root.right, n22)
    root.right.right = attach(root.right, n35)

    s = successor(n22)
    print(s.val if s else "null")  # 30
