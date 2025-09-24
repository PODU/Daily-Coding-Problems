// Cartesian tree (min-heap + in-order == input) built with O(n) monotonic stack; then verify in-order and pretty-print.
// Time: O(n), Space: O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node left, right;
        Node(int v) { val = v; }
    }

    static Node buildCartesian(int[] s) {
        Deque<Node> stk = new ArrayDeque<>();
        for (int v : s) {
            Node cur = new Node(v);
            Node last = null;
            while (!stk.isEmpty() && stk.peekLast().val > v) {
                last = stk.pollLast();
            }
            cur.left = last;
            if (!stk.isEmpty()) stk.peekLast().right = cur;
            stk.addLast(cur);
        }
        return stk.isEmpty() ? null : stk.peekFirst();
    }

    static void inorder(Node n, List<Integer> out) {
        if (n == null) return;
        inorder(n.left, out);
        out.add(n.val);
        inorder(n.right, out);
    }

    public static void main(String[] args) {
        int[] s = {3, 2, 6, 1, 9};
        Node root = buildCartesian(s);
        List<Integer> io = new ArrayList<>();
        inorder(root, io);
        if (!io.equals(Arrays.asList(3, 2, 6, 1, 9)))
            throw new AssertionError("in-order mismatch");
        System.out.println("      1");
        System.out.println("    /   \\");
        System.out.println("  2       9");
        System.out.println(" / \\");
        System.out.println("3   6");
    }
}
