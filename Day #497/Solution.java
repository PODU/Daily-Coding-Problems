// Convert binary tree to FULL binary tree by removing nodes with exactly one child.
// Post-order recursion: a one-child node is replaced by its processed child.
// Time: O(n), Space: O(h) recursion.
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node fullTree(Node root) {
        if (root == null) return null;
        root.left = fullTree(root.left);
        root.right = fullTree(root.right);
        if (root.left != null && root.right == null) return root.left;
        if (root.left == null && root.right != null) return root.right;
        return root;
    }

    static void preorder(Node root, StringBuilder sb) {
        if (root == null) return;
        if (sb.length() > 0) sb.append(' ');
        sb.append(root.val);
        preorder(root.left, sb);
        preorder(root.right, sb);
    }

    public static void main(String[] args) {
        Node root = new Node(0);
        root.left = new Node(1);
        root.right = new Node(2);
        root.left.left = new Node(3);
        root.left.left.right = new Node(5);
        root.right.right = new Node(4);
        root.right.right.left = new Node(6);
        root.right.right.right = new Node(7);

        root = fullTree(root);
        StringBuilder sb = new StringBuilder();
        preorder(root, sb);
        System.out.println(sb.toString());
    }
}
