// Height-balanced binary tree check.
// Bottom-up DFS returning subtree height, or -1 if unbalanced. O(n) time, O(h) space.
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int check(Node root) {
        if (root == null) return 0;
        int l = check(root.left);
        if (l == -1) return -1;
        int r = check(root.right);
        if (r == -1) return -1;
        if (Math.abs(l - r) > 1) return -1;
        return Math.max(l, r) + 1;
    }

    static boolean isBalanced(Node root) {
        return check(root) != -1;
    }

    public static void main(String[] args) {
        Node a = new Node(1);
        a.left = new Node(2);
        a.right = new Node(3);
        a.left.left = new Node(4);

        Node b = new Node(1);
        b.left = new Node(2);
        b.left.left = new Node(3);

        System.out.println("Balanced tree: " + isBalanced(a));
        System.out.println("Unbalanced tree: " + isBalanced(b));
    }
}
