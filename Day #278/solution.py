# Day 278: Generate all structurally distinct BSTs with N nodes (values 1..N).
# Recursive divide on root choice. Count = Catalan(N). Time O(Catalan(N)*N).


class Node:
    __slots__ = ("val", "left", "right")

    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def build(lo, hi):
    if lo > hi:
        return [None]
    res = []
    for r in range(lo, hi + 1):
        for L in build(lo, r - 1):
            for R in build(r + 1, hi):
                n = Node(r)
                n.left, n.right = L, R
                res.append(n)
    return res


def preorder(n):
    if n is None:
        return "#"
    return f"{n.val} {preorder(n.left)} {preorder(n.right)}"


if __name__ == "__main__":
    N = 3
    trees = build(1, N)
    print("Count:", len(trees))  # 5
    for t in trees:
        print(preorder(t))
