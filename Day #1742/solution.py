# Day 1742: Approach: recursive generation of all BSTs; root choice + Cartesian product of left/right.
# Time/Space O(Catalan(N)*N).
class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build(lo, hi):
    if lo > hi:
        return [None]
    res = []
    for r in range(lo, hi + 1):
        lefts = build(lo, r - 1)
        rights = build(r + 1, hi)
        for l in lefts:
            for ri in rights:
                root = Node(r)
                root.left = l
                root.right = ri
                res.append(root)
    return res


def preorder(node, out):
    if node is None:
        return
    out.append(node.val)
    preorder(node.left, out)
    preorder(node.right, out)


if __name__ == "__main__":
    N = 3
    trees = build(1, N)
    print(len(trees))
    for t in trees:
        out = []
        preorder(t, out)
        print(",".join(map(str, out)))
