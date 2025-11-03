# Day 545: LCA with parent pointers: equalize depths, then walk both up together.
# Time O(h), Space O(1).
class Node:
    def __init__(self, val):
        self.val = val
        self.parent = None
        self.left = None
        self.right = None


def depth(n):
    d = 0
    while n:
        d += 1
        n = n.parent
    return d


def lca(a, b):
    da, db = depth(a), depth(b)
    while da > db:
        a = a.parent
        da -= 1
    while db > da:
        b = b.parent
        db -= 1
    while a is not b:
        a = a.parent
        b = b.parent
    return a


def child(parent, c):
    if c:
        c.parent = parent
    return c


def main():
    n3, n5, n1 = Node(3), Node(5), Node(1)
    n6, n2, n0, n8 = Node(6), Node(2), Node(0), Node(8)
    n7, n4 = Node(7), Node(4)

    n3.left = child(n3, n5)
    n3.right = child(n3, n1)
    n5.left = child(n5, n6)
    n5.right = child(n5, n2)
    n1.left = child(n1, n0)
    n1.right = child(n1, n8)
    n2.left = child(n2, n7)
    n2.right = child(n2, n4)

    print(lca(n6, n4).val)
    print(lca(n6, n8).val)


if __name__ == "__main__":
    main()
