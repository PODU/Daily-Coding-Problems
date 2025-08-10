// Day 94: Max path sum in a binary tree. DFS returns best downward gain; at each
// node consider a path bending through it. O(n) time, O(h) space.
public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
        Node(int v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static int best = Integer.MIN_VALUE;

    static int gain(Node n) {
        if (n == null) return 0;
        int l = Math.max(gain(n.left), 0);
        int r = Math.max(gain(n.right), 0);
        best = Math.max(best, n.val + l + r);
        return n.val + Math.max(l, r);
    }

    public static void main(String[] args) {
        Node root = new Node(-10, new Node(9),
            new Node(20, new Node(15), new Node(7)));
        gain(root);
        System.out.println(best); // 42
    }
}
