# Day 702: Serialize/deserialize a binary tree.
# Approach: preorder traversal with '#' null markers, comma-separated tokens.
# Both directions O(n) time and space.


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def serialize(root):
    out = []

    def go(n):
        if n is None:
            out.append("#")
            return
        out.append(str(n.val))
        go(n.left)
        go(n.right)

    go(root)
    return ",".join(out)


def deserialize(s):
    toks = iter(s.split(","))

    def go():
        t = next(toks)
        if t == "#":
            return None
        n = Node(t)
        n.left = go()
        n.right = go()
        return n

    return go()


if __name__ == "__main__":
    node = Node("root", Node("left", Node("left.left")), Node("right"))
    assert deserialize(serialize(node)).left.left.val == "left.left"
    print(deserialize(serialize(node)).left.left.val)  # left.left
