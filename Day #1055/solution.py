# Day 1055: Inorder successor in a BST using parent pointers.
# If node has right subtree -> leftmost of right subtree; else walk up parents
# until coming from a left child. Time O(h), Space O(1).


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None
        self.parent = None


def inorder_successor(node):
    if node is None:
        return None
    if node.right:
        cur = node.right
        while cur.left:
            cur = cur.left
        return cur
    cur = node
    p = node.parent
    while p and p.right is cur:
        cur = p
        p = p.parent
    return p


def main():
    root = Node(10)
    n5 = Node(5)
    n30 = Node(30)
    n22 = Node(22)
    n35 = Node(35)
    root.left, n5.parent = n5, root
    root.right, n30.parent = n30, root
    n30.left, n22.parent = n22, n30
    n30.right, n35.parent = n35, n30

    succ = inorder_successor(n22)
    print(succ.val if succ else "null")


if __name__ == "__main__":
    main()
