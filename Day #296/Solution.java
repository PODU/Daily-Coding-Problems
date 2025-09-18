// Sorted array -> height-balanced BST: pick lower-mid as root, recurse halves; print BFS level-order.
// Time O(n), Space O(log n) recursion.
import java.util.*;

public class Solution {
    static class Node { int val; Node l, r; Node(int v){ val = v; } }

    static Node build(int[] a, int lo, int hi) {
        if (lo > hi) return null;
        int mid = (lo + hi) / 2;
        Node n = new Node(a[mid]);
        n.l = build(a, lo, mid - 1);
        n.r = build(a, mid + 1, hi);
        return n;
    }

    public static void main(String[] args) {
        int[] a = {-10, -3, 0, 5, 9};
        Node root = build(a, 0, a.length - 1);
        List<Integer> order = new ArrayList<>();
        Queue<Node> q = new LinkedList<>();
        q.add(root);
        while (!q.isEmpty()) {
            Node n = q.poll();
            if (n == null) continue;
            order.add(n.val);
            q.add(n.l); q.add(n.r);
        }
        System.out.println(order.toString());
    }
}
