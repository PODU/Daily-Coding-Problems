# Day 471: Generate all structurally distinct BSTs with values 1..N via recursive root selection.
# Time: O(Catalan(N) * N), Space: O(Catalan(N) * N) for the trees.


class Node:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def generate(start, end):
    if start > end:
        return [None]
    trees = []
    for i in range(start, end + 1):
        lefts = generate(start, i - 1)
        rights = generate(i + 1, end)
        for l in lefts:
            for r in rights:
                root = Node(i)
                root.left = l
                root.right = r
                trees.append(root)
    return trees


def preorder(root, out):
    if root is None:
        return
    out.append(root.val)
    preorder(root.left, out)
    preorder(root.right, out)


def main():
    N = 3
    trees = generate(1, N)
    print("Number of BSTs: " + str(len(trees)))
    for t in trees:
        out = []
        preorder(t, out)
        print(" ".join(str(x) for x in out))


if __name__ == "__main__":
    main()
