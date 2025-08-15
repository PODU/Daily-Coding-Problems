# Day 125: Two nodes in a BST summing to K.
# Inorder traversal -> sorted values, two-pointer. O(n) time, O(n) space.


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def inorder(r, out):
    if not r:
        return
    inorder(r.left, out)
    out.append(r.val)
    inorder(r.right, out)


def two_sum(root, k):
    v = []
    inorder(root, v)
    i, j = 0, len(v) - 1
    while i < j:
        s = v[i] + v[j]
        if s == k:
            return (v[i], v[j])
        if s < k:
            i += 1
        else:
            j -= 1
    return None


if __name__ == "__main__":
    root = Node(10, Node(5), Node(15, Node(11), Node(15)))
    a, b = two_sum(root, 20)
    print("Return the nodes %d and %d." % (a, b))
