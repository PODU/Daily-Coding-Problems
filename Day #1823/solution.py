# Day 1823: Convert to full binary tree by removing single-child nodes (post-order recursion).
# O(N) time, O(H) space.
class Node:
    def __init__(self, val):
        self.val = val
        self.l = None
        self.r = None


def fullify(n):
    if n is None:
        return None
    n.l = fullify(n.l)
    n.r = fullify(n.r)
    if n.l is None and n.r is not None:
        return n.r
    if n.l is not None and n.r is None:
        return n.l
    return n


def serialize(n):
    if n is None:
        return ""
    if n.l is None and n.r is None:
        return str(n.val)
    return "{}({}, {})".format(n.val, serialize(n.l), serialize(n.r))


if __name__ == "__main__":
    nodes = [Node(i) for i in range(8)]
    nodes[0].l, nodes[0].r = nodes[1], nodes[2]
    nodes[1].l = nodes[3]
    nodes[2].r = nodes[4]
    nodes[3].r = nodes[5]
    nodes[4].l, nodes[4].r = nodes[6], nodes[7]
    root = fullify(nodes[0])
    print(serialize(root))  # 0(5, 4(6, 7))
