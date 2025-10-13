// Day 426: Level of binary tree with minimum sum (levels 0-indexed; root = level 0).
// BFS level-order summing each level, track minimum. Time O(n), Space O(width).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2);
        root.right = new Node(3);

        Queue<Node> q = new LinkedList<>();
        q.add(root);
        int level = 0, bestLevel = 0;
        long best = Long.MAX_VALUE;
        while (!q.isEmpty()) {
            int sz = q.size();
            long s = 0;
            for (int i = 0; i < sz; i++) {
                Node n = q.poll();
                s += n.val;
                if (n.left != null) q.add(n.left);
                if (n.right != null) q.add(n.right);
            }
            if (s < best) {
                best = s;
                bestLevel = level;
            }
            level++;
        }
        System.out.println("Level with minimum sum: " + bestLevel + " (sum = " + best + ")");
    }
}
