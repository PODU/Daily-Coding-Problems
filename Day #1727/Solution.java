// Level of a binary tree with the minimum node-value sum.
// BFS level-order, track the level whose sum is smallest. O(n) time, O(w) space (max width).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static int minSumLevel(Node root) {
        if (root == null) return -1;
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        int level = 0, bestLevel = 0;
        long bestSum = Long.MAX_VALUE;
        while (!q.isEmpty()) {
            int sz = q.size();
            long sum = 0;
            for (int i = 0; i < sz; i++) {
                Node n = q.poll();
                sum += n.val;
                if (n.left != null) q.add(n.left);
                if (n.right != null) q.add(n.right);
            }
            if (sum < bestSum) { bestSum = sum; bestLevel = level; }
            level++;
        }
        return bestLevel;
    }

    public static void main(String[] args) {
        Node root = new Node(5);
        root.left = new Node(2);
        root.right = new Node(3);
        root.left.left = new Node(-5);
        System.out.println("Level with minimum sum: " + minSumLevel(root));
    }
}
