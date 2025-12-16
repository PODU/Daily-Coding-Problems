// Day 752: Level-order (BFS) traversal of a binary tree.
// Time: O(n), Space: O(w) where w is the max width of the tree.
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static List<Integer> levelOrder(Node root) {
        List<Integer> out = new ArrayList<>();
        if (root == null) return out;
        Queue<Node> q = new ArrayDeque<>();
        q.add(root);
        while (!q.isEmpty()) {
            Node n = q.poll();
            out.add(n.val);
            if (n.left != null) q.add(n.left);
            if (n.right != null) q.add(n.right);
        }
        return out;
    }

    public static void main(String[] args) {
        Node root = new Node(1);
        root.left = new Node(2); root.right = new Node(3);
        root.right.left = new Node(4); root.right.right = new Node(5);

        List<Integer> res = levelOrder(root);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < res.size(); i++) {
            sb.append(res.get(i));
            if (i + 1 < res.size()) sb.append(", ");
        }
        System.out.println(sb.toString());
    }
}
