# Day 3: Serialize/deserialize a binary tree via preorder traversal with '#' null markers.
# Time: O(n) for both, Space: O(n).
class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def serialize(root):
    out = []

    def rec(n):
        if n is None:
            out.append("#")
            return
        out.append(str(n.val))  # assumes values contain no ','
        rec(n.left)
        rec(n.right)

    rec(root)
    return ",".join(out)


def deserialize(s):
    toks = iter(s.split(","))

    def rec():
        t = next(toks)
        if t == "#":
            return None
        return Node(t, rec(), rec())

    return rec()


if __name__ == "__main__":
    node = Node("root", Node("left", Node("left.left")), Node("right"))
    assert deserialize(serialize(node)).left.left.val == "left.left"
    print(deserialize(serialize(node)).left.left.val)
