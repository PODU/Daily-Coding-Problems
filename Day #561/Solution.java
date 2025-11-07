// Sorted array -> height-balanced BST: recurse on mid=(lo+hi)/2 as root.
// Time: O(N), Space: O(N) for nodes + O(log N) recursion.
import java.util.ArrayDeque;
import java.util.Queue;

public class Solution {
    static class TreeNode {
        int val;
        TreeNode left, right;
        TreeNode(int v) { val = v; }
    }

    static TreeNode build(int[] a, int lo, int hi) {
        if (lo > hi) return null;
        int mid = (lo + hi) / 2;
        TreeNode root = new TreeNode(a[mid]);
        root.left = build(a, lo, mid - 1);
        root.right = build(a, mid + 1, hi);
        return root;
    }

    public static void main(String[] args) {
        int[] a = {1, 2, 3, 4, 5, 6, 7};
        TreeNode root = build(a, 0, a.length - 1);
        StringBuilder sb = new StringBuilder();
        Queue<TreeNode> q = new ArrayDeque<>();
        if (root != null) q.add(root);
        while (!q.isEmpty()) {
            TreeNode n = q.poll();
            if (sb.length() > 0) sb.append(' ');
            sb.append(n.val);
            if (n.left != null) q.add(n.left);
            if (n.right != null) q.add(n.right);
        }
        System.out.println(sb.toString());
    }
}
