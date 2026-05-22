// Day 1546: Stable partition of a linked list around pivot k.
// Build two sublists (< k) and (>= k) preserving order, then splice. Time O(n), Space O(1).
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node partition(Node head, int k) {
        Node lessD = new Node(0), geD = new Node(0);
        Node l = lessD, g = geD;
        for (Node c = head; c != null; c = c.next) {
            if (c.val < k) { l.next = c; l = c; }
            else           { g.next = c; g = c; }
        }
        g.next = null;
        l.next = geD.next;
        return lessD.next;
    }

    public static void main(String[] args) {
        int[] vals = {5, 1, 8, 0, 3};
        Node head = null, tail = null;
        for (int v : vals) {
            Node n = new Node(v);
            if (head == null) head = tail = n; else { tail.next = n; tail = n; }
        }
        head = partition(head, 3);
        StringBuilder sb = new StringBuilder();
        for (Node c = head; c != null; c = c.next) {
            if (sb.length() > 0) sb.append(" -> ");
            sb.append(c.val);
        }
        System.out.println(sb);
    }
}
