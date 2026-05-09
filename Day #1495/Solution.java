// Day 1495: Build a min-heap-ordered Cartesian tree whose in-order traversal is S.
// Approach: monotonic stack; pop nodes > current as its left subtree. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static Node buildCartesian(int[] s) {
        Deque<Node> st = new ArrayDeque<>();
        Node root = null;
        for (int x : s) {
            Node cur = new Node(x), last = null;
            while (!st.isEmpty() && st.peek().val > x) last = st.pop();
            cur.left = last;
            if (!st.isEmpty()) st.peek().right = cur;
            else root = cur;
            st.push(cur);
        }
        // root is the bottom of stack; track separately
        if (root == null && !st.isEmpty()) root = st.peekLast();
        return root != null ? root : (st.isEmpty() ? null : st.peekLast());
    }

    static void inorder(Node n, List<Integer> out) {
        if (n == null) return;
        inorder(n.left, out); out.add(n.val); inorder(n.right, out);
    }

    static void pretty(Node n, int depth) {
        if (n == null) return;
        pretty(n.right, depth + 1);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < depth * 4; i++) sb.append(' ');
        System.out.println(sb.toString() + n.val);
        pretty(n.left, depth + 1);
    }

    static void listing(Node n) {
        if (n == null) return;
        if (n.left != null || n.right != null) {
            StringBuilder sb = new StringBuilder("  " + n.val + " -> ");
            if (n.left != null) sb.append(n.left.val).append(" ");
            if (n.right != null) sb.append(n.right.val);
            System.out.println(sb.toString());
        }
        listing(n.left); listing(n.right);
    }

    public static void main(String[] args) {
        int[] s = {3, 2, 6, 1, 9};
        Node root = buildCartesian(s);

        List<Integer> io = new ArrayList<>();
        inorder(root, io);
        StringBuilder sb = new StringBuilder("In-order traversal:");
        for (int v : io) sb.append(" ").append(v);
        System.out.println(sb.toString());

        System.out.println("Root: " + root.val);
        System.out.println("Parent -> children:");
        listing(root);

        System.out.println("Tree (rotated 90 deg, root at left):");
        pretty(root, 0);
    }
}
