# Day 644: Count unival subtrees (all nodes share one value).
# Approach: post-order DFS; a node is unival iff both children are unival and
# their values match the node's. Count as we recurse.
# Time: O(n), Space: O(h).
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
        left = dfs(node.left)
        right = dfs(node.right)
        if not left or not right:
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
    root = Node(0,
                Node(1),
                Node(0,
                     Node(1, Node(1), Node(1)),
                     Node(0)))
    print(count_unival(root))  # 5
