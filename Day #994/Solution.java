// Day 994: Print binary tree nodes level by level (BFS).
// Use a queue; dequeue a node, emit it, enqueue its children. O(n) time/space.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
        Node(int v, Node l, Node r) { val = v; left = l; right = r; }
    }

    static List<Integer> levelOrder(Node root) {
        List<Integer> out = new ArrayList<>();
        Queue<Node> q = new ArrayDeque<>();
        if (root != null) q.add(root);
        while (!q.isEmpty()) {
            Node n = q.poll();
            out.add(n.val);
            if (n.left != null) q.add(n.left);
            if (n.right != null) q.add(n.right);
        }
        return out;
    }

    public static void main(String[] args) {
        Node root = new Node(1, new Node(2), new Node(3, new Node(4), new Node(5)));
        StringBuilder b = new StringBuilder();
        List<Integer> v = levelOrder(root);
        for (int i = 0; i < v.size(); i++) {
            if (i > 0) b.append(", ");
            b.append(v.get(i));
        }
        System.out.println(b); // 1, 2, 3, 4, 5
    }
}
