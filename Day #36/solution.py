# Day 36: Second largest in BST: walk right to largest; second largest is parent of
# largest (if no left subtree) else max of largest's left subtree. O(h) time, O(1) space.
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def insert(root, v):
    if root is None:
        return Node(v)
    if v < root.val:
        root.left = insert(root.left, v)
    else:
        root.right = insert(root.right, v)
    return root


def second_largest(root):
    parent, cur = None, root
    while cur.right:
        parent, cur = cur, cur.right
    if cur.left:
        cur = cur.left
        while cur.right:
            cur = cur.right
        return cur.val
    return parent.val


if __name__ == "__main__":
    root = None
    for v in [5, 3, 8, 2, 4, 7, 9]:
        root = insert(root, v)
    print(second_largest(root))
