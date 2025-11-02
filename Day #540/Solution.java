// Day 540: Boustrophedon (zigzag) level-order traversal of a binary tree.
// BFS level by level, reversing every other level. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> boustrophedon(Node root) {
        List<Integer> out = new ArrayList<>();
        if (root == null) return out;
        Deque<Node> q = new ArrayDeque<>();
        q.add(root);
        boolean leftToRight = true;
        while (!q.isEmpty()) {
            int sz = q.size();
            Integer[] level = new Integer[sz];
            for (int i = 0; i < sz; i++) {
                Node n = q.poll();
                int idx = leftToRight ? i : sz - 1 - i;
                level[idx] = n.val;
                if (n.left != null) q.add(n.left);
                if (n.right != null) q.add(n.right);
            }
            out.addAll(Arrays.asList(level));
            leftToRight = !leftToRight;
        }
        return out;
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2); root.right = new Node(3);
        root.left.left = new Node(4); root.left.right = new Node(5);
        root.right.left = new Node(6); root.right.right = new Node(7);
        System.out.println(boustrophedon(root));
    }
}
