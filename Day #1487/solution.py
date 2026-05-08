# Day 1487: In-order traversal in O(1) space via Morris traversal.
# Time: O(n). Space: O(1) extra.

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
            pred = cur.left
            while pred.right and pred.right is not cur:
                pred = pred.right
            if pred.right is None:
                pred.right = cur        # create thread
                cur = cur.left
            else:
                pred.right = None       # restore tree
                out.append(cur.val)
                cur = cur.right
    return out


if __name__ == "__main__":
    #        4
    #       / \
    #      2   6
    #     / \ / \
    #    1  3 5  7
    root = Node(4,
                Node(2, Node(1), Node(3)),
                Node(6, Node(5), Node(7)))
    print("In-order:", " ".join(map(str, morris_inorder(root))))
