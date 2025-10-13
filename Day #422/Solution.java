// Day 422: Merge two binary trees recursively (value = sum), O(n) time, O(h) space.
// Then print merged tree by level-order traversal (skipping null children).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node merge(Node a, Node b) {
        if (a == null) return b;
        if (b == null) return a;
        Node n = new Node(a.val + b.val);
        n.left = merge(a.left, b.left);
        n.right = merge(a.right, b.right);
        return n;
    }

    public static void main(String[] args) {
        Node t1 = new Node(1);
        t1.left = new Node(3); t1.right = new Node(2);
        t1.left.left = new Node(5);

        Node t2 = new Node(2);
        t2.left = new Node(1); t2.right = new Node(3);
        t2.left.right = new Node(4);
        t2.right.right = new Node(7);

        Node m = merge(t1, t2);

        Queue<Node> q = new LinkedList<>(); q.add(m);
        StringBuilder sb = new StringBuilder();
        while (!q.isEmpty()) {
            Node c = q.poll();
            if (sb.length() > 0) sb.append(" ");
            sb.append(c.val);
            if (c.left != null) q.add(c.left);
            if (c.right != null) q.add(c.right);
        }
        System.out.println(sb);
    }
}
