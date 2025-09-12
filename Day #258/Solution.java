// Day 258: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing the output order on alternate levels.
// Time: O(n), Space: O(n).
import java.util.ArrayDeque;
import java.util.ArrayList;
import java.util.List;
import java.util.Queue;

public class Solution {
    static class TreeNode {
        int val;
        TreeNode left, right;
        TreeNode(int v) { val = v; }
    }

    static List<Integer> boustrophedon(TreeNode root) {
        List<Integer> out = new ArrayList<>();
        if (root == null) return out;
        Queue<TreeNode> q = new ArrayDeque<>();
        q.add(root);
        boolean leftToRight = true;
        while (!q.isEmpty()) {
            int sz = q.size();
            Integer[] level = new Integer[sz];
            for (int i = 0; i < sz; i++) {
                TreeNode node = q.poll();
                int idx = leftToRight ? i : sz - 1 - i;
                level[idx] = node.val;
                if (node.left != null) q.add(node.left);
                if (node.right != null) q.add(node.right);
            }
            for (int v : level) out.add(v);
            leftToRight = !leftToRight;
        }
        return out;
    }

    public static void main(String[] args) {
        TreeNode root = new TreeNode(1);
        root.left = new TreeNode(2);
        root.right = new TreeNode(3);
        root.left.left = new TreeNode(4);
        root.left.right = new TreeNode(5);
        root.right.left = new TreeNode(6);
        root.right.right = new TreeNode(7);
        System.out.println(boustrophedon(root)); // [1, 3, 2, 4, 5, 6, 7]
    }
}
