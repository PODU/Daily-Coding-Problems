// Zigzag (boustrophedon) level order of a binary tree. BFS per level, reverse alternate levels. O(N) time, O(N) space.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> zigzag(Node root) {
        List<Integer> res = new ArrayList<>();
        if (root == null) return res;
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        boolean leftToRight = true;
        while (!q.isEmpty()) {
            int sz = q.size();
            Integer[] level = new Integer[sz];
            for (int i = 0; i < sz; i++) {
                Node cur = q.poll();
                int idx = leftToRight ? i : sz - 1 - i;
                level[idx] = cur.val;
                if (cur.left != null) q.add(cur.left);
                if (cur.right != null) q.add(cur.right);
            }
            res.addAll(Arrays.asList(level));
            leftToRight = !leftToRight;
        }
        return res;
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2);
        root.right = new Node(3);
        root.left.left = new Node(4);
        root.left.right = new Node(5);
        root.right.left = new Node(6);
        root.right.right = new Node(7);

        System.out.println(zigzag(root));
    }
}
