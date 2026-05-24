// Merge two binary trees by summing overlapping nodes; recurse and reuse the
// non-null subtree when only one side exists. Time O(n), Space O(h).
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
        t1.left = new Node(3);
        t1.right = new Node(2);
        t1.left.left = new Node(5);

        Node t2 = new Node(2);
        t2.left = new Node(1);
        t2.right = new Node(3);
        t2.left.right = new Node(4);
        t2.right.right = new Node(7);

        Node m = merge(t1, t2);

        List<String> out = new ArrayList<>();
        Queue<Node> q = new LinkedList<>();
        q.add(m);
        while (!q.isEmpty()) {
            Node cur = q.poll();
            if (cur != null) {
                out.add(String.valueOf(cur.val));
                q.add(cur.left);
                q.add(cur.right);
            } else {
                out.add("null");
            }
        }
        while (!out.isEmpty() && out.get(out.size() - 1).equals("null"))
            out.remove(out.size() - 1);

        System.out.println(String.join(" ", out));
    }
}
