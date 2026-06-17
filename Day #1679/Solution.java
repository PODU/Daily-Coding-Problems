// Day 1679: Size of largest BST subtree. Post-order returns (isBST, size, min, max)
// per subtree, tracking the global best. Time O(n), Space O(h).
public class Solution {
    static class TreeNode { int val; TreeNode left, right; TreeNode(int v) { val = v; } }
    static class Info { boolean isBST; int size; long mn, mx; Info(boolean b, int s, long mn, long mx) { isBST = b; size = s; this.mn = mn; this.mx = mx; } }

    static int best = 0;

    static Info dfs(TreeNode node) {
        if (node == null) return new Info(true, 0, Long.MAX_VALUE, Long.MIN_VALUE);
        Info l = dfs(node.left), r = dfs(node.right);
        if (l.isBST && r.isBST && l.mx < node.val && node.val < r.mn) {
            int sz = l.size + r.size + 1;
            best = Math.max(best, sz);
            return new Info(true, sz, Math.min(node.val, l.mn), Math.max(node.val, r.mx));
        }
        return new Info(false, 0, Long.MIN_VALUE, Long.MAX_VALUE);
    }

    static int largestBST(TreeNode root) { best = 0; dfs(root); return best; }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(10);
        root.left = new TreeNode(5);
        root.right = new TreeNode(15);
        root.left.left = new TreeNode(1);
        root.left.right = new TreeNode(8);
        root.right.right = new TreeNode(7);
        System.out.println(largestBST(root)); // 3
    }
}
