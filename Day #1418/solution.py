# Day 1418: inorder successor of a BST node using parent pointers.
# Approach: if right subtree exists, leftmost of it; else climb until node is a left child. O(h).


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None
        self.parent = None


def successor(node):
    if node.right:
        cur = node.right
        while cur.left:
            cur = cur.left
        return cur
    cur = node
    while cur.parent and cur is cur.parent.right:
        cur = cur.parent
    return cur.parent


def attach(parent, child):
    if child:
        child.parent = parent
    return child


if __name__ == "__main__":
    root = Node(10)
    root.left = attach(root, Node(5))
    root.right = attach(root, Node(30))
    n22 = Node(22)
    root.right.left = attach(root.right, n22)
    root.right.right = attach(root.right, Node(35))

    s = successor(n22)
    print(s.val if s else "null")  # 30
