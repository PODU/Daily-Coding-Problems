// Day 442: Cartesian tree (min-heap ordered, in-order == S) built with a
// monotonic stack in O(n) time, O(n) space.
import java.util.*;

public class Solution {
    static class Node {
        int val; Node left, right;
        Node(int v) { val = v; }
    }

    static Node buildCartesian(int[] s) {
        Deque<Node> stack = new ArrayDeque<>();
        for (int v : s) {
            Node node = new Node(v);
            Node last = null;
            while (!stack.isEmpty() && stack.peek().val > v) last = stack.pop();
            node.left = last;
            if (!stack.isEmpty()) stack.peek().right = node;
            stack.push(node);
        }
        return stack.isEmpty() ? null : stack.peekLast();
    }

    static void print(Node n, String prefix, String tag) {
        if (n == null) return;
        System.out.println(prefix + tag + n.val);
        print(n.left, prefix + "  ", "L-- ");
        print(n.right, prefix + "  ", "R-- ");
    }

    public static void main(String[] args) {
        int[] s = {3, 2, 6, 1, 9};
        Node root = buildCartesian(s);
        print(root, "", "");
        // 1 / L-- 2 (L-- 3, R-- 6) / R-- 9
    }
}
