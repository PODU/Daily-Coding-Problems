# Day 1619: Subtree check via serialization with sentinels + substring search.
# Time: O(n+m), Space: O(n+m).


class TreeNode:
    def __init__(self, val):
        self.val = val
        self.left = None
        self.right = None


def serialize(node):
    if node is None:
        return ",#"
    return "," + str(node.val) + serialize(node.left) + serialize(node.right)


def is_subtree(s, t):
    return serialize(t) in serialize(s)


def main():
    s = TreeNode(3)
    s.left = TreeNode(4)
    s.right = TreeNode(5)
    s.left.left = TreeNode(1)
    s.left.right = TreeNode(2)

    t = TreeNode(4)
    t.left = TreeNode(1)
    t.right = TreeNode(2)

    print("true" if is_subtree(s, t) else "false")


if __name__ == "__main__":
    main()
