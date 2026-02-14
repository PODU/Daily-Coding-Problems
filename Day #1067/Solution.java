// Post-order prune: remove subtrees consisting entirely of zeros. Time: O(n), Space: O(n) stack.
import java.util.*;

public class Solution {
    static class TreeNode {
        int val; TreeNode left, right;
        TreeNode(int v) { val = v; }
        TreeNode(int v, TreeNode l, TreeNode r) { val = v; left = l; right = r; }
    }

    static TreeNode prune(TreeNode n) {
        if (n == null) return null;
        n.left  = prune(n.left);
        n.right = prune(n.right);
        if (n.val == 0 && n.left == null && n.right == null) return null;
        return n;
    }

    static void preorder(TreeNode n, List<String> out) {
        if (n == null) { out.add("X"); return; }
        out.add(String.valueOf(n.val));
        preorder(n.left,  out);
        preorder(n.right, out);
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(0,
            new TreeNode(1),
            new TreeNode(0,
                new TreeNode(1, new TreeNode(0), new TreeNode(0)),
                new TreeNode(0)
            )
        );
        root = prune(root);
        List<String> out = new ArrayList<>();
        preorder(root, out);
        System.out.println("Pruned preorder (X=null): " + String.join(" ", out));
    }
}
