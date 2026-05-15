// Count unival subtrees (all nodes in subtree share one value).
// Approach: post-order DFS; a node is unival iff children are unival and equal in value.
// Time: O(n), Space: O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int count = 0;

    static boolean dfs(Node node) {
        if (node == null) return true;
        boolean l = dfs(node.left);
        boolean r = dfs(node.right);
        if (!l || !r) return false;
        if (node.left != null && node.left.val != node.val) return false;
        if (node.right != null && node.right.val != node.val) return false;
        count++;
        return true;
    }

    static int countUnivalSubtrees(Node root) {
        count = 0;
        dfs(root);
        return count;
    }

    public static void main(String[] args) {
        Node root = new Node(0);
        root.left = new Node(1);
        root.right = new Node(0);
        root.right.left = new Node(1);
        root.right.right = new Node(0);
        root.right.left.left = new Node(1);
        root.right.left.right = new Node(1);
        System.out.println(countUnivalSubtrees(root)); // 5
    }
}
