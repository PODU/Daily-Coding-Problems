// Day 596: Invert a binary tree (mirror it).
// Approach: recursively swap left/right children. Time O(n), Space O(h).
import java.util.*;

public class Solution {
    static class Node {
        char val;
        Node left, right;
        Node(char v) { val = v; }
    }

    static void invert(Node root) {
        if (root == null) return;
        Node tmp = root.left;
        root.left = root.right;
        root.right = tmp;
        invert(root.left);
        invert(root.right);
    }

    public static void main(String[] args) {
        Node a = new Node('a'), b = new Node('b'), c = new Node('c');
        Node d = new Node('d'), e = new Node('e'), f = new Node('f');
        a.left = b; a.right = c;
        b.left = d; b.right = e;
        c.left = f;

        invert(a);

        Queue<Node> q = new LinkedList<>();
        q.add(a);
        StringBuilder out = new StringBuilder();
        while (!q.isEmpty()) {
            int sz = q.size();
            StringBuilder line = new StringBuilder();
            for (int i = 0; i < sz; i++) {
                Node n = q.poll();
                if (line.length() > 0) line.append(' ');
                line.append(n.val);
                if (n.left != null) q.add(n.left);
                if (n.right != null) q.add(n.right);
            }
            out.append(line).append('\n');
        }
        System.out.print(out);
    }
}
