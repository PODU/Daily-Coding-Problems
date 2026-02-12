# Day 1057: Generate all distinct BSTs with values 1..N: recursively pick each value as
# root, combine all left/right subtree shapes. Count is Catalan(N).
# Time/Space O(Catalan(N) * N).


class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build(lo, hi):
    if lo > hi:
        return [None]
    res = []
    for root in range(lo, hi + 1):
        lefts = build(lo, root - 1)
        rights = build(root + 1, hi)
        for l in lefts:
            for r in rights:
                node = TreeNode(root)
                node.left = l
                node.right = r
                res.append(node)
    return res


def preorder(node):
    if node is None:
        return "#"
    return "{} {} {}".format(node.val, preorder(node.left), preorder(node.right))


def main():
    N = 3
    trees = build(1, N)
    print(len(trees))  # 5 for N=3
    for t in trees:
        print(preorder(t))


if __name__ == "__main__":
    main()
