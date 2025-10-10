// Largest BST subtree: bottom-up DFS returning {isBST, size, min, max}; combine children.
// Time O(n), Space O(h).

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static class Info {
        boolean isBST; int size, min, max;
        Info(boolean b, int s, int mn, int mx) { isBST = b; size = s; min = mn; max = mx; }
    }

    static int best = 0;

    static Info dfs(Node node) {
        if (node == null) return new Info(true, 0, Integer.MAX_VALUE, Integer.MIN_VALUE);
        Info l = dfs(node.left), r = dfs(node.right);
        if (l.isBST && r.isBST && node.val > l.max && node.val < r.min) {
            int sz = l.size + r.size + 1;
            best = Math.max(best, sz);
            return new Info(true, sz, Math.min(node.val, l.min), Math.max(node.val, r.max));
        }
        return new Info(false, 0, 0, 0);
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.left = new Node(5);
        root.right = new Node(15);
        root.left.left = new Node(1);
        root.left.right = new Node(8);
        root.right.right = new Node(7);
        dfs(root);
        System.out.println(best);
    }
}
