# Day 808: In-order traversal of a binary tree using O(1) extra space (Morris).
# Thread predecessor's right pointer to current, then unthread. Time O(N), Space O(1).


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
            pred = cur.left
            while pred.right and pred.right is not cur:
                pred = pred.right
            if pred.right is None:          # create thread
                pred.right = cur
                cur = cur.left
            else:                            # thread exists -> visit
                pred.right = None
                out.append(cur.val)
                cur = cur.right
    return out


if __name__ == "__main__":
    root = Node(4)
    root.left, root.right = Node(2), Node(6)
    root.left.left, root.left.right = Node(1), Node(3)
    root.right.left, root.right.right = Node(5), Node(7)
    print(morris_inorder(root))  # [1, 2, 3, 4, 5, 6, 7]
