// BFS level-order: sum each level, track the level (1-indexed) with min sum.
// Time O(n), Space O(width).
import java.util.*;

public class Solution {
    static class Node { int val; Node l, r; Node(int v){val=v;} }

    static int[] minSumLevel(Node root) {
        if (root == null) return new int[]{-1, 0};
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        int level = 0, bestLevel = 1;
        long bestSum = Long.MAX_VALUE;
        while (!q.isEmpty()) {
            int sz = q.size();
            long sum = 0;
            level++;
            for (int i = 0; i < sz; i++) {
                Node n = q.poll();
                sum += n.val;
                if (n.l != null) q.add(n.l);
                if (n.r != null) q.add(n.r);
            }
            if (sum < bestSum) { bestSum = sum; bestLevel = level; }
        }
        return new int[]{bestLevel, (int) bestSum};
    }

    public static void main(String[] args) {
        Node root = new Node(10);
        root.l = new Node(2); root.r = new Node(3);
        root.l.l = new Node(4); root.l.r = new Node(5);
        int[] r = minSumLevel(root);
        System.out.println("Level with minimum sum: " + r[0] + " (sum = " + r[1] + ")");
    }
}
