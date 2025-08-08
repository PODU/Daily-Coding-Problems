// Day 83: Invert (mirror) a binary tree by swapping children recursively.
// Time O(n), Space O(h).
import java.util.*;

public class Solution {
    static class Node {
        char val; Node left, right;
        Node(char v) { val = v; }
    }

    static Node invert(Node root) {
        if (root == null) return null;
        Node tmp = root.left; root.left = root.right; root.right = tmp;
        invert(root.left);
        invert(root.right);
        return root;
    }

    static String levelOrder(Node root) {
        StringBuilder sb = new StringBuilder();
        Queue<Node> q = new LinkedList<>();
        if (root != null) q.add(root);
        while (!q.isEmpty()) {
            Node n = q.poll();
            sb.append(n.val).append(' ');
            if (n.left != null) q.add(n.left);
            if (n.right != null) q.add(n.right);
        }
        return sb.toString().trim();
    }

    public static void main(String[] args) {
        Node a = new Node('a');
        a.left = new Node('b'); a.right = new Node('c');
        a.left.left = new Node('d'); a.left.right = new Node('e');
        a.right.left = new Node('f');
        System.out.println("before: " + levelOrder(a)); // a b c d e f
        invert(a);
        System.out.println("after:  " + levelOrder(a)); // a c b f e d
    }
}
