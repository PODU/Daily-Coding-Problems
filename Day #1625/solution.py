# Day 1625: Inorder successor in BST using parent pointers.
# If right subtree exists, leftmost of it; else climb until node is left child. O(h).
class Node:
    def __init__(self, val):
        self.val = val
        self.left = self.right = self.parent = None


def inorder_successor(node):
    if node is None:
        return None
    if node.right:
        cur = node.right
        while cur.left:
            cur = cur.left
        return cur
    cur = node
    while cur.parent and cur.parent.right is cur:
        cur = cur.parent
    return cur.parent


def insert(root, v):
    if root is None:
        return Node(v)
    cur = root
    while True:
        if v < cur.val:
            if not cur.left:
                cur.left = Node(v); cur.left.parent = cur; return root
            cur = cur.left
        else:
            if not cur.right:
                cur.right = Node(v); cur.right.parent = cur; return root
            cur = cur.right


def find(root, v):
    while root and root.val != v:
        root = root.left if v < root.val else root.right
    return root


if __name__ == "__main__":
    root = None
    for v in [10, 5, 30, 22, 35]:
        root = insert(root, v)
    s = inorder_successor(find(root, 22))
    print(f"The inorder successor of 22 is {s.val if s else 'None'}.")
