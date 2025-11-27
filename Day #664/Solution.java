// Day 664: Maximum path sum between any two nodes in a binary tree.
// Post-order DFS: each node returns best downward gain; track best "bridge". Time O(n), Space O(h).
public class Solution {
    static class Node { int val; Node l, r; Node(int v) { val = v; } }

    static int best;
    static int gain(Node n) {
        if (n == null) return 0;
        int lg = Math.max(0, gain(n.l));
        int rg = Math.max(0, gain(n.r));
        best = Math.max(best, n.val + lg + rg);
        return n.val + Math.max(lg, rg);
    }
    static int maxPathSum(Node root) { best = Integer.MIN_VALUE; gain(root); return best; }

    public static void main(String[] args) {
        Node root = new Node(-10);
        root.l = new Node(9);
        root.r = new Node(20);
        root.r.l = new Node(15);
        root.r.r = new Node(7);
        System.out.println(maxPathSum(root)); // 42
    }
}
