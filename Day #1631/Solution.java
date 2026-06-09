// Validate BST via recursive inclusive min/max bounds (left<=root<=right). O(n) time, O(h) space.
public class Solution {
    static class Node {
        long val;
        Node left, right;
        Node(long v) { val = v; }
    }

    static boolean isValid(Node n, long lo, long hi) {
        if (n == null) return true;
        if (n.val < lo || n.val > hi) return false;
        return isValid(n.left, lo, n.val) && isValid(n.right, n.val, hi);
    }

    static boolean validate(Node root) {
        return isValid(root, Long.MIN_VALUE, Long.MAX_VALUE);
    }

    public static void main(String[] args) {
        // Valid BST: root 5, left 3 (2,4), right 8 (6,9)
        Node root = new Node(5);
        root.left = new Node(3);
        root.left.left = new Node(2);
        root.left.right = new Node(4);
        root.right = new Node(8);
        root.right.left = new Node(6);
        root.right.right = new Node(9);

        // Invalid: root 5, left child 6
        Node bad = new Node(5);
        bad.left = new Node(6);

        System.out.println(validate(root) ? "true" : "false");
        System.out.println(validate(bad) ? "true" : "false");
    }
}
