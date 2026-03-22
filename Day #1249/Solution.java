// Validate BST with inclusive bounds (left <= node <= right) via recursive range check. O(n) time, O(h) space.
public class Solution {
    static class Node {
        long val;
        Node left, right;
        Node(long v) { val = v; }
    }

    static boolean isValid(Node node, long lo, long hi) {
        if (node == null) return true;
        if (node.val < lo || node.val > hi) return false;
        return isValid(node.left, lo, node.val) && isValid(node.right, node.val, hi);
    }

    static boolean isValidBST(Node root) {
        return isValid(root, Long.MIN_VALUE, Long.MAX_VALUE);
    }

    public static void main(String[] args) {
        // Valid tree: root 5, left 3 (2,5), right 8 (5,12)
        Node root = new Node(5);
        root.left = new Node(3);
        root.left.left = new Node(2);
        root.left.right = new Node(5);
        root.right = new Node(8);
        root.right.left = new Node(5);
        root.right.right = new Node(12);
        System.out.println(isValidBST(root) ? "true" : "false");

        // Invalid tree: root 5, left 6 (6 > 5 violates)
        Node bad = new Node(5);
        bad.left = new Node(6);
        bad.right = new Node(8);
        System.out.println(isValidBST(bad) ? "true" : "false");
    }
}
