// Day 1606: Check if a binary tree is height-balanced.
// Bottom-up recursion returning height, -1 if unbalanced. Time O(n), Space O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int check(Node root) { // height, or -1 if unbalanced
        if (root == null) return 0;
        int l = check(root.left);
        if (l == -1) return -1;
        int r = check(root.right);
        if (r == -1) return -1;
        if (Math.abs(l - r) > 1) return -1;
        return Math.max(l, r) + 1;
    }

    static boolean isBalanced(Node root) { return check(root) != -1; }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2);
        root.right = new Node(3);
        root.left.left = new Node(4);
        System.out.println(isBalanced(root) ? "true" : "false"); // true
    }
}
