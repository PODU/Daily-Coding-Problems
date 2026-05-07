// Day 1486: Partition a linked list around pivot k (stable).
// Approach: build two sublists (< k and >= k), then concatenate. O(n) time, O(1) extra space.
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node partition(Node head, int k) {
        Node lessDummy = new Node(0), geDummy = new Node(0);
        Node less = lessDummy, ge = geDummy;
        for (Node cur = head; cur != null; cur = cur.next) {
            if (cur.val < k) { less.next = cur; less = cur; }
            else { ge.next = cur; ge = cur; }
        }
        ge.next = null;
        less.next = geDummy.next;
        return lessDummy.next;
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
            sb.append(c.val);
            if (c.next != null) sb.append(" -> ");
        }
        System.out.println(sb);
    }
}
