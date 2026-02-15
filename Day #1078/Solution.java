// Postorder DFS: each node returns val+max(0,L,R) upward; global best = val+max(0,L)+max(0,R); O(n) time O(h) space
public class Solution {
    static int best;

    static class TreeNode {
        int val;
        TreeNode left, right;
        TreeNode(int v) { val = v; }
    }

    static int dfs(TreeNode node) {
        if (node == null) return 0;
        int l = Math.max(0, dfs(node.left));
        int r = Math.max(0, dfs(node.right));
        best = Math.max(best, node.val + l + r);
        return node.val + Math.max(l, r);
    }

    static int maxPathSum(TreeNode root) {
        best = Integer.MIN_VALUE;
        dfs(root);
        return best;
    }

    public static void main(String[] args) {
        //       -10
        //       /  \
        //      9    20
        //          /  \
        //         15   7
        TreeNode root1 = new TreeNode(-10);
        root1.left = new TreeNode(9);
        root1.right = new TreeNode(20);
        root1.right.left = new TreeNode(15);
        root1.right.right = new TreeNode(7);
        System.out.println("Max path sum: " + maxPathSum(root1));

        //    1
        //   / \
        //  2   3
        TreeNode root2 = new TreeNode(1);
        root2.left = new TreeNode(2);
        root2.right = new TreeNode(3);
        System.out.println("Max path sum: " + maxPathSum(root2));
    }
}
