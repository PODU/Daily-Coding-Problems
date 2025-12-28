// Day 810: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> boustrophedon(Node root) {
        List<Integer> out = new ArrayList<>();
        if (root == null) return out;
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        boolean ltr = true;
        while (!q.isEmpty()) {
            int sz = q.size();
            LinkedList<Integer> level = new LinkedList<>();
            for (int i = 0; i < sz; i++) {
                Node n = q.poll();
                if (ltr) level.addLast(n.val); else level.addFirst(n.val);
                if (n.left != null) q.add(n.left);
                if (n.right != null) q.add(n.right);
            }
            out.addAll(level);
            ltr = !ltr;
        }
        return out;
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2); root.right = new Node(3);
        root.left.left = new Node(4); root.left.right = new Node(5);
        root.right.left = new Node(6); root.right.right = new Node(7);
        System.out.println(boustrophedon(root)); // [1, 3, 2, 4, 5, 6, 7]
    }
}
