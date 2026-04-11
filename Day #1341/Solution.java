// Check height-balanced binary tree via bottom-up DFS; -1 sentinel marks unbalanced.
// Time O(n), Space O(h).
public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int height(Node root) {
        if (root == null) return 0;
        int l = height(root.left);
        if (l == -1) return -1;
        int r = height(root.right);
        if (r == -1) return -1;
        if (Math.abs(l - r) > 1) return -1;
        return Math.max(l, r) + 1;
    }

    static boolean isBalanced(Node root) {
        return height(root) != -1;
    }

    public static void main(String[] args) {
        // Balanced tree [1,2,3,4,5,null,6]
        Node a = new Node(1);
        a.left = new Node(2);
        a.right = new Node(3);
        a.left.left = new Node(4);
        a.left.right = new Node(5);
        a.right.right = new Node(6);
        System.out.println("Balanced: " + isBalanced(a));

        // Skewed tree 1 -> 2 -> 3
        Node b = new Node(1);
        b.left = new Node(2);
        b.left.left = new Node(3);
        System.out.println("Balanced: " + isBalanced(b));
    }
}
