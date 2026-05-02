// Invert binary tree by swapping children recursively; print level-order (BFS).
// Time O(n), Space O(n).
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

        StringBuilder sb = new StringBuilder();
        Queue<Node> q = new LinkedList<>();
        q.add(a);
        while (!q.isEmpty()) {
            Node n = q.poll();
            if (n == null) continue;
            if (sb.length() > 0) sb.append(' ');
            sb.append(n.val);
            q.add(n.left);
            q.add(n.right);
        }
        System.out.println(sb.toString());
    }
}
