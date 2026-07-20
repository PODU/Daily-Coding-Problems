# Day 1846: Serialize/deserialize a binary tree via preorder traversal with null markers.
# Time O(N), Space O(N). Uses JSON-style nested lists for safe round-tripping.
import json


class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def serialize(root):
    def build(n):
        if n is None:
            return None
        return [n.val, build(n.left), build(n.right)]

    return json.dumps(build(root))


def deserialize(s):
    def build(data):
        if data is None:
            return None
        val, left, right = data
        return Node(val, build(left), build(right))

    return build(json.loads(s))


def main():
    node = Node("root", Node("left", Node("left.left")), Node("right"))
    assert deserialize(serialize(node)).left.left.val == "left.left"
    print(deserialize(serialize(node)).left.left.val == "left.left")


if __name__ == "__main__":
    main()
