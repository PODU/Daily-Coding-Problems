// Merge two binary trees recursively (sum overlaps, keep lone nodes). O(min(n1,n2)) time.
// Serialize merged tree as BFS level-order with trailing nulls trimmed.
import java.util.ArrayList;
import java.util.List;

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

    static String serialize(Node root) {
        List<String> out = new ArrayList<>();
        // Use an index-walked list as the BFS queue so it can hold null children.
        List<Node> queue = new ArrayList<>();
        queue.add(root);
        int i = 0;
        while (i < queue.size()) {
            Node n = queue.get(i++);
            if (n == null) { out.add("null"); continue; }
            out.add(Integer.toString(n.val));
            queue.add(n.left);
            queue.add(n.right);
        }
        while (!out.isEmpty() && out.get(out.size() - 1).equals("null"))
            out.remove(out.size() - 1);
        return "[" + String.join(", ", out) + "]";
    }

    public static void main(String[] args) {
        Node t1 = new Node(1);
        t1.left = new Node(3); t1.right = new Node(2);
        t1.left.left = new Node(5);

        Node t2 = new Node(2);
        t2.left = new Node(1); t2.right = new Node(3);
        t2.left.right = new Node(4);
        t2.right.right = new Node(7);

        System.out.println(serialize(merge(t1, t2)));
    }
}
