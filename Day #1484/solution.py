# Day 1484: Construct all structurally unique BSTs with N nodes (values 1..N).
# For each root i, recursively combine all left BSTs of (lo..i-1) with all right
# BSTs of (i+1..hi). Count is the Catalan number C(N).
# Time/Space O(Catalan(N) * N).

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def build(lo, hi):
    if lo > hi:
        return [None]
    trees = []
    for i in range(lo, hi + 1):
        for l in build(lo, i - 1):
            for r in build(i + 1, hi):
                trees.append(Node(i, l, r))
    return trees


def preorder(node):
    if node is None:
        return []
    return [node.val] + preorder(node.left) + preorder(node.right)


def all_bsts(n):
    return build(1, n) if n > 0 else [None]


if __name__ == "__main__":
    trees = all_bsts(3)
    print(len(trees))  # 5
    for t in trees:
        print(preorder(t))
