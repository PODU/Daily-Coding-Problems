# Day 997: Serialize / deserialize a binary tree.
# Preorder traversal with "#" markers for null children, comma-joined.
# Both serialize and deserialize are O(n) time and space.

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
    it = iter(s.split(","))

    def go():
        v = next(it)
        if v == "#":
            return None
        n = Node(v)
        n.left = go()
        n.right = go()
        return n

    return go()


if __name__ == "__main__":
    node = Node('root', Node('left', Node('left.left')), Node('right'))
    s = serialize(node)
    print(s)
    assert deserialize(serialize(node)).left.left.val == 'left.left'
    print("assertion passed:", deserialize(s).left.left.val)
