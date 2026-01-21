// Day 936: Sum of BST values within inclusive range [a,b], pruning branches out of range.
// Time O(n) worst, O(h + k) typical, Space O(h).
public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static int rangeSum(Node n, int a, int b) {
        if (n == null) return 0;
        if (n.val < a) return rangeSum(n.right, a, b);
        if (n.val > b) return rangeSum(n.left, a, b);
        return n.val + rangeSum(n.left, a, b) + rangeSum(n.right, a, b);
    }

    public static void main(String[] args) {
        Node r = new Node(5);
        r.left = new Node(3); r.right = new Node(8);
        r.left.left = new Node(2); r.left.right = new Node(4);
        r.right.left = new Node(6); r.right.right = new Node(10);
        System.out.println(rangeSum(r, 4, 9)); // 23
    }
}
