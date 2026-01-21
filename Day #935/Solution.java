// Day 935: Check if a binary tree is height-balanced.
// Bottom-up DFS returning height, -1 signals imbalance. Time O(n), Space O(h).
public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    // Returns height if balanced, else -1.
    static int check(Node n) {
        if (n == null) return 0;
        int l = check(n.left);
        if (l == -1) return -1;
        int r = check(n.right);
        if (r == -1) return -1;
        if (Math.abs(l - r) > 1) return -1;
        return 1 + Math.max(l, r);
    }
    static boolean isBalanced(Node root) { return check(root) != -1; }

    public static void main(String[] args) {
        Node a = new Node(1); a.left = new Node(2); a.right = new Node(3);
        a.left.left = new Node(4);
        System.out.println(isBalanced(a)); // true

        Node b = new Node(1); b.left = new Node(2);
        b.left.left = new Node(3); b.left.left.left = new Node(4);
        System.out.println(isBalanced(b)); // false
    }
}
