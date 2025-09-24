// Merge two binary trees recursively (sum overlaps, keep lone nodes); print merged tree level-order skipping nulls.
// Time: O(n), Space: O(n).
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

        Queue<Node> q = new LinkedList<>();
        q.add(m);
        while (!q.isEmpty()) {
            int sz = q.size();
            StringBuilder line = new StringBuilder();
            for (int i = 0; i < sz; i++) {
                Node cur = q.poll();
                if (i > 0) line.append(" ");
                line.append(cur.val);
                if (cur.left != null) q.add(cur.left);
                if (cur.right != null) q.add(cur.right);
            }
            System.out.println(line.toString());
        }
    }
}
