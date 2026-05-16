# Day 1523: Second largest in BST via parent-walk: find largest; if it has a left subtree,
# answer = max of that subtree, else answer = parent of largest. Time O(h), Space O(1).
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


def max_node(n):
    while n.right:
        n = n.right
    return n.val


def second_largest(root):
    cur, parent = root, None
    while cur.right:
        parent = cur
        cur = cur.right
    if cur.left:
        return max_node(cur.left)
    return parent.val


if __name__ == "__main__":
    root = None
    for v in [5, 3, 8, 2, 4, 7, 9]:
        root = insert(root, v)
    print(second_largest(root))
