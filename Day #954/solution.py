# Day 954: count unival subtrees (all nodes in subtree share one value).
# Post-order DFS, returning whether the subtree is unival. Time O(n), Space O(h).

class Node:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


def count_unival(root):
    count = 0

    def dfs(node):
        nonlocal count
        if node is None:
            return True
        l = dfs(node.left)
        r = dfs(node.right)
        if not l or not r:
            return False
        if node.left and node.left.val != node.val:
            return False
        if node.right and node.right.val != node.val:
            return False
        count += 1
        return True

    dfs(root)
    return count


if __name__ == "__main__":
    root = Node(0, Node(1), Node(0, Node(1, Node(1), Node(1)), Node(0)))
    print(count_unival(root))  # 5
