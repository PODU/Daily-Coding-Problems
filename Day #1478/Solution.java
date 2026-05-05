// Day 1478: Return a deepest node of a binary tree.
// Single DFS returning (depth, node); keep the deeper subtree's result.
// Time O(N), Space O(H).
public class Solution {
    static class Node {
        char val;
        Node left, right;
        Node(char v) { val = v; }
        Node(char v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static Node deepestNode;
    static int deepestDepth;

    static int dfs(Node node, int depth) {
        if (node == null) return depth;
        if (node.left == null && node.right == null) {
            if (depth > deepestDepth) { deepestDepth = depth; deepestNode = node; }
            return depth;
        }
        dfs(node.left, depth + 1);
        dfs(node.right, depth + 1);
        return depth;
    }

    public static void main(String[] args) {
        Node root = new Node('a', new Node('b', new Node('d'), null), new Node('c'));
        deepestDepth = -1;
        dfs(root, 0);
        System.out.println(deepestNode.val);  // d
    }
}
