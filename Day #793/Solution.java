// Prune binary tree to a full binary tree: post-order recursion; if a node has
// exactly one child, replace it with its (already pruned) child. O(n) time, O(h) space.
import java.util.LinkedList;
import java.util.Queue;

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
        if (root.left != null && root.right == null) return root.left;
        if (root.left == null && root.right != null) return root.right;
        return root;
    }

    public static void main(String[] args) {
        Node n0 = new Node(0), n1 = new Node(1), n2 = new Node(2), n3 = new Node(3);
        Node n4 = new Node(4), n5 = new Node(5), n6 = new Node(6), n7 = new Node(7);
        n0.left = n1; n0.right = n2;
        n1.left = n3;
        n2.right = n4;
        n3.right = n5;
        n4.left = n6; n4.right = n7;

        Node root = prune(n0);

        Queue<Node> q = new LinkedList<>();
        q.add(root);
        StringBuilder sb = new StringBuilder();
        while (!q.isEmpty()) {
            int sz = q.size();
            for (int i = 0; i < sz; i++) {
                Node cur = q.poll();
                sb.append(cur.val);
                if (i + 1 < sz) sb.append(' ');
                if (cur.left != null) q.add(cur.left);
                if (cur.right != null) q.add(cur.right);
            }
            sb.append('\n');
        }
        System.out.print(sb);
    }
}
