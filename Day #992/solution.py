# Day 992: Second largest node in a BST.
# Walk the rightmost path; the largest is the rightmost node. The 2nd largest is
# either its left subtree's maximum, or its parent. O(h) time, O(1) extra space.

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def insert(root, val):
    if root is None:
        return Node(val)
    if val < root.val:
        root.left = insert(root.left, val)
    else:
        root.right = insert(root.right, val)
    return root


def second_largest(root):
    if root is None or (root.left is None and root.right is None):
        return None
    cur, parent = root, None
    while cur.right is not None:        # go to the largest (rightmost)
        parent, cur = cur, cur.right
    if cur.left is not None:            # largest has a left subtree -> its max
        cur = cur.left
        while cur.right is not None:
            cur = cur.right
        return cur.val
    return parent.val                  # else parent of the largest


if __name__ == "__main__":
    root = None
    for v in [5, 3, 8, 2, 4, 7, 9]:
        root = insert(root, v)
    print(second_largest(root))        # 8
