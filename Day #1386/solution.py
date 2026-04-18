# Day 1386: Second largest in BST via reverse in-order (right,node,left), stop at 2nd visited node. O(h) space, O(n) worst time.

class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def second_largest(root):
    st = []
    cur = root
    count = 0
    while cur or st:
        while cur:
            st.append(cur)
            cur = cur.right
        cur = st.pop()
        count += 1
        if count == 2:
            return cur.val
        cur = cur.left
    return -1


if __name__ == "__main__":
    root = Node(5)
    root.left = Node(3)
    root.left.left = Node(2)
    root.left.right = Node(4)
    root.right = Node(8)
    root.right.left = Node(7)
    root.right.right = Node(9)
    print(second_largest(root))
