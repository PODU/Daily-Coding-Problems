// Prune binary tree: remove subtrees containing only 0s (no 1 descendant).
// Post-order recursion. O(n) time, O(h) recursion stack.
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node prune(Node root) {
        if (root == null) return null;
        root.left = prune(root.left);
        root.right = prune(root.right);
        if (root.val == 0 && root.left == null && root.right == null) return null;
        return root;
    }

    static void printLevelOrder(Node root) {
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        List<String> out = new ArrayList<>();
        while (!q.isEmpty()) {
            Node n = q.poll();
            if (n == null) { out.add("null"); continue; }
            out.add(String.valueOf(n.val));
            q.add(n.left);
            q.add(n.right);
        }
        while (!out.isEmpty() && out.get(out.size() - 1).equals("null")) out.remove(out.size() - 1);
        System.out.println("[" + String.join(", ", out) + "]");
    }

    public static void main(String[] args) {
        Node root = new Node(0);
        root.left = new Node(1);
        root.right = new Node(0);
        root.right.left = new Node(1);
        root.right.right = new Node(0);
        root.right.left.left = new Node(0);
        root.right.left.right = new Node(0);

        root = prune(root);
        printLevelOrder(root); // [0, 1, 0, null, null, 1]
    }
}
