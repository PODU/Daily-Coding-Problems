# Day 1784: Morris in-order traversal: thread tree via predecessor links for O(1) space.
# Time O(N), Space O(1) (excluding output).
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None

def morris_inorder(root):
    out = []
    cur = root
    while cur:
        if not cur.left:
            out.append(cur.val)
            cur = cur.right
        else:
            pre = cur.left
            while pre.right and pre.right is not cur:
                pre = pre.right
            if pre.right is None:
                pre.right = cur
                cur = cur.left
            else:
                pre.right = None
                out.append(cur.val)
                cur = cur.right
    print(' '.join(map(str, out)))

if __name__ == '__main__':
    root = Node(4)
    root.left = Node(2)
    root.right = Node(5)
    root.left.left = Node(1)
    root.left.right = Node(3)
    morris_inorder(root)
