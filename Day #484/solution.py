# Day 484: Second largest node in a BST.
# O(h): walk right to largest; second largest is its parent, or max of largest's left subtree.
# Time O(h), Space O(1).
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def insert(root, v):
    if not root:
        return Node(v)
    if v < root.val:
        root.left = insert(root.left, v)
    else:
        root.right = insert(root.right, v)
    return root


def second_largest(root):
    if not root or (not root.left and not root.right):
        return None
    cur, parent = root, None
    while cur.right:
        parent, cur = cur, cur.right
    if cur.left:  # largest has a left subtree -> max of it
        cur = cur.left
        while cur.right:
            cur = cur.right
        return cur
    return parent  # parent of the largest


if __name__ == "__main__":
    root = None
    for v in [5, 3, 8, 2, 4, 7, 10]:
        root = insert(root, v)
    print(second_largest(root).val)  # 8
