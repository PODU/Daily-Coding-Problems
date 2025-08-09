// Day 89: Validate BST via recursive [lo, hi] range check (left<=root<=right allowed).
// Time O(n), Space O(h).
public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static boolean valid(Node n, long lo, long hi) {
        if (n == null) return true;
        if (n.val < lo || n.val > hi) return false;
        return valid(n.left, lo, n.val) && valid(n.right, n.val, hi);
    }

    static boolean isBST(Node root) { return valid(root, Long.MIN_VALUE, Long.MAX_VALUE); }

    public static void main(String[] args) {
        Node a = new Node(5);
        a.left = new Node(3); a.right = new Node(8);
        a.left.left = new Node(2); a.left.right = new Node(4);
        System.out.println(isBST(a)); // true

        Node b = new Node(5);
        b.left = new Node(3); b.right = new Node(8);
        b.left.right = new Node(6); // invalid
        System.out.println(isBST(b)); // false
    }
}
