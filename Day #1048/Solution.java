// Cartesian tree (min-heap ordered, in-order = S) via linear stack on right spine.
// Build O(n); rotated-90 print + verification. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node build(int[] s) {
        Deque<Node> st = new ArrayDeque<>();
        for (int v : s) {
            Node cur = new Node(v);
            Node last = null;
            while (!st.isEmpty() && st.peekLast().val > v) {
                last = st.pollLast();
            }
            cur.left = last;
            if (!st.isEmpty()) st.peekLast().right = cur;
            st.addLast(cur);
        }
        return st.isEmpty() ? null : st.peekFirst();
    }

    static void printRotated(Node n, int depth) {
        if (n == null) return;
        printRotated(n.right, depth + 1);
        StringBuilder sb = new StringBuilder();
        for (int i = 0; i < depth * 4; i++) sb.append(' ');
        System.out.println(sb.toString() + n.val);
        printRotated(n.left, depth + 1);
    }

    static void inorder(Node n, List<Integer> out) {
        if (n == null) return;
        inorder(n.left, out);
        out.add(n.val);
        inorder(n.right, out);
    }

    static boolean minHeap(Node n) {
        if (n == null) return true;
        if (n.left != null && n.left.val <= n.val) return false;
        if (n.right != null && n.right.val <= n.val) return false;
        return minHeap(n.left) && minHeap(n.right);
    }

    public static void main(String[] args) {
        int[] s = {3, 2, 6, 1, 9};
        Node root = build(s);
        System.out.println("Cartesian tree (rotated 90 deg, root=" + root.val + "):");
        printRotated(root, 0);
        List<Integer> in = new ArrayList<>();
        inorder(root, in);
        StringBuilder sb = new StringBuilder("in-order:");
        for (int x : in) sb.append(" ").append(x);
        System.out.println(sb.toString());
        System.out.println("min-heap ordered: " + minHeap(root));
    }
}
