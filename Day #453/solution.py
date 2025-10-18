# Day 453: Two nodes in a BST summing to K.
# Inorder -> sorted list, then two-pointer. Time O(n), Space O(n).


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def inorder(n, out):
    if not n:
        return
    inorder(n.left, out)
    out.append(n.val)
    inorder(n.right, out)


def two_sum(root, k):
    a = []
    inorder(root, a)
    i, j = 0, len(a) - 1
    while i < j:
        s = a[i] + a[j]
        if s == k:
            return a[i], a[j]
        if s < k:
            i += 1
        else:
            j -= 1
    return None


if __name__ == "__main__":
    root = Node(10, Node(5), Node(15, Node(11), Node(15)))
    x, y = two_sum(root, 20)
    print(f"{x} and {y}")  # 5 and 15
