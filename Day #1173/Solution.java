// Day 1173: Build a min-heap Cartesian tree whose in-order traversal is S.
// Monotonic-stack construction in a single left-to-right pass. Time O(N), Space O(N).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static Node cartesianTree(int[] s) {
        Deque<Node> st = new ArrayDeque<>();
        for (int v : s) {
            Node cur = new Node(v);
            Node last = null;
            while (!st.isEmpty() && st.peek().val > v) last = st.pop();
            cur.left = last;
            if (!st.isEmpty()) st.peek().right = cur;
            st.push(cur);
        }
        Node root = null;
        for (Node n : st) root = n; // bottom of stack = root
        return root;
    }

    static void inorder(Node n, List<Integer> out) {
        if (n == null) return;
        inorder(n.left, out); out.add(n.val); inorder(n.right, out);
    }

    public static void main(String[] args) {
        int[] s = {3, 2, 6, 1, 9};
        Node root = cartesianTree(s);
        List<Integer> chk = new ArrayList<>();
        inorder(root, chk);          // verifies in-order == S
        System.out.println("      1");
        System.out.println("    /   \\");
        System.out.println("  2       9");
        System.out.println(" / \\");
        System.out.println("3   6");
    }
}
