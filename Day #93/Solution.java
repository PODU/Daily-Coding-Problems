// Day 93: Largest BST subtree size. Post-order DFS returning (isBST, size, min,
// max) per node and combining children. O(n) time, O(h) space.
public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
        Node(int v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static int best = 0;

    // returns {isBst(1/0), size, min, max}
    static int[] dfs(Node n) {
        if (n == null) return new int[]{1, 0, Integer.MAX_VALUE, Integer.MIN_VALUE};
        int[] l = dfs(n.left), r = dfs(n.right);
        if (l[0] == 1 && r[0] == 1 && l[3] < n.val && n.val < r[2]) {
            int sz = l[1] + r[1] + 1;
            best = Math.max(best, sz);
            return new int[]{1, sz, Math.min(l[2], n.val), Math.max(r[3], n.val)};
        }
        return new int[]{0, 0, 0, 0};
    }

    public static void main(String[] args) {
        Node root = new Node(10,
            new Node(5, new Node(1), new Node(8)),
            new Node(15, null, new Node(7)));
        dfs(root);
        System.out.println(best); // 3
    }
}
