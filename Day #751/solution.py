# Day 751: In-order traversal with O(1) extra space via Morris Traversal.
# Time: O(n), Space: O(1) (re-uses null right pointers as temporary threads).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def morris_inorder(root):
    out = []
    cur = root
    while cur:
        if cur.left is None:
            out.append(cur.val)
            cur = cur.right
        else:
            pre = cur.left
            while pre.right is not None and pre.right is not cur:
                pre = pre.right
            if pre.right is None:        # create thread
                pre.right = cur
                cur = cur.left
            else:                        # remove thread and visit
                pre.right = None
                out.append(cur.val)
                cur = cur.right
    return out


if __name__ == "__main__":
    #        4
    #      /   \
    #     2     6
    #    / \   / \
    #   1   3 5   7
    root = Node(4,
                Node(2, Node(1), Node(3)),
                Node(6, Node(5), Node(7)))
    print(" ".join(map(str, morris_inorder(root))))
