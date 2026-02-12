// Generate all distinct BSTs with values 1..N: recursively pick each value as
// root, combine all left/right subtree shapes. Count is Catalan(N).
// Time/Space O(Catalan(N) * N).
import java.util.ArrayList;
import java.util.List;

public class Solution {
    static class TreeNode {
        int val;
        TreeNode left, right;
        TreeNode(int v) { val = v; }
    }

    static List<TreeNode> build(int lo, int hi) {
        List<TreeNode> res = new ArrayList<>();
        if (lo > hi) {
            res.add(null);
            return res;
        }
        for (int root = lo; root <= hi; root++) {
            List<TreeNode> lefts = build(lo, root - 1);
            List<TreeNode> rights = build(root + 1, hi);
            for (TreeNode l : lefts) {
                for (TreeNode r : rights) {
                    TreeNode node = new TreeNode(root);
                    node.left = l;
                    node.right = r;
                    res.add(node);
                }
            }
        }
        return res;
    }

    static void preorder(TreeNode node, StringBuilder out) {
        if (node == null) { out.append("#"); return; }
        out.append(node.val).append(" ");
        preorder(node.left, out);
        out.append(" ");
        preorder(node.right, out);
    }

    public static void main(String[] args) {
        int N = 3;
        List<TreeNode> trees = build(1, N);
        System.out.println(trees.size()); // 5 for N=3
        for (TreeNode t : trees) {
            StringBuilder sb = new StringBuilder();
            preorder(t, sb);
            System.out.println(sb.toString());
        }
    }
}
