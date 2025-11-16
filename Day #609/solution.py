# Day 609: Inorder successor in a BST using parent pointers.
# Approach: if right child exists, take its leftmost; else climb until coming from a left child. Time O(h).


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
        c = node.right
        while c.left:
            c = c.left
        return c
    cur, p = node, node.parent
    while p and cur is p.right:
        cur, p = p, p.parent
    return p


def attach(parent, child):
    if child:
        child.parent = parent
    return child


def main():
    n10, n5, n30 = Node(10), Node(5), Node(30)
    n22, n35 = Node(22), Node(35)
    n10.left = attach(n10, n5)
    n10.right = attach(n10, n30)
    n30.left = attach(n30, n22)
    n30.right = attach(n30, n35)

    s = inorder_successor(n22)
    print(s.val if s else "null")  # 30


if __name__ == '__main__':
    main()
