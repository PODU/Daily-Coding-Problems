// BFS level-order traversal, track sum per level; return 1-indexed level with min sum. O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static int[] minSumLevel(Node root) {
        if (root == null) return new int[]{-1, 0};
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        int level = 1, minLevel = 1;
        long minSum = Long.MAX_VALUE;
        while (!q.isEmpty()) {
            int sz = q.size(); long sum = 0;
            for (int i = 0; i < sz; i++) {
                Node cur = q.poll();
                sum += cur.val;
                if (cur.left  != null) q.add(cur.left);
                if (cur.right != null) q.add(cur.right);
            }
            if (sum < minSum) { minSum = sum; minLevel = level; }
            level++;
        }
        return new int[]{minLevel, (int) minSum};
    }

    public static void main(String[] args) {
        // Tree 1
        Node r1 = new Node(1);
        r1.left = new Node(2); r1.right = new Node(3);
        r1.left.left = new Node(4); r1.left.right = new Node(5);
        r1.right.right = new Node(6);
        int[] res1 = minSumLevel(r1);
        System.out.println("Level with min sum: " + res1[0] + " (sum=" + res1[1] + ")");

        // Tree 2
        Node r2 = new Node(10);
        r2.left = new Node(2); r2.right = new Node(3);
        int[] res2 = minSumLevel(r2);
        System.out.println("Level with min sum: " + res2[0] + " (sum=" + res2[1] + ")");
    }
}
