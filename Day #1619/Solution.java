// Subtree check via serialization with sentinels + substring search.
// Time: O(n+m), Space: O(n+m).
public class Solution {
    static class TreeNode {
        int val;
        TreeNode left, right;
        TreeNode(int v) { val = v; }
    }

    static void serialize(TreeNode node, StringBuilder sb) {
        if (node == null) { sb.append(",#"); return; }
        sb.append(",").append(node.val);
        serialize(node.left, sb);
        serialize(node.right, sb);
    }

    static boolean isSubtree(TreeNode s, TreeNode t) {
        StringBuilder ss = new StringBuilder(), ts = new StringBuilder();
        serialize(s, ss);
        serialize(t, ts);
        return ss.toString().contains(ts.toString());
    }

    public static void main(String[] args) {
        TreeNode s = new TreeNode(3);
        s.left = new TreeNode(4);
        s.right = new TreeNode(5);
        s.left.left = new TreeNode(1);
        s.left.right = new TreeNode(2);

        TreeNode t = new TreeNode(4);
        t.left = new TreeNode(1);
        t.right = new TreeNode(2);

        System.out.println(isSubtree(s, t) ? "true" : "false");
    }
}
