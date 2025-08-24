// Bottom-up (iterative) merge sort on a singly linked list. O(n log n) time, O(1) space.
public class Solution {
    static class Node { int val; Node next; Node(int v){ val = v; } }

    static Node[] merge(Node a, Node b) {
        Node dummy = new Node(0), tail = dummy;
        while (a != null && b != null) {
            if (a.val <= b.val) { tail.next = a; a = a.next; }
            else { tail.next = b; b = b.next; }
            tail = tail.next;
        }
        tail.next = (a != null) ? a : b;
        while (tail.next != null) tail = tail.next;
        return new Node[]{ dummy.next, tail };
    }

    static int length(Node h) { int n = 0; while (h != null){ n++; h = h.next; } return n; }

    static Node split(Node head, int n) {
        for (int i = 1; head != null && i < n; i++) head = head.next;
        if (head == null) return null;
        Node rest = head.next; head.next = null; return rest;
    }

    static Node sortList(Node head) {
        int n = length(head);
        Node dummy = new Node(0); dummy.next = head;
        for (int size = 1; size < n; size <<= 1) {
            Node cur = dummy.next, tail = dummy;
            while (cur != null) {
                Node left = cur;
                Node right = split(left, size);
                cur = split(right, size);
                Node[] m = merge(left, right);
                tail.next = m[0]; tail = m[1];
            }
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        int[] vals = {4, 1, -3, 99};
        Node dummy = new Node(0), t = dummy;
        for (int v : vals) { t.next = new Node(v); t = t.next; }
        Node head = sortList(dummy.next);
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        for (Node p = head; p != null; p = p.next) { if (!first) sb.append(" -> "); sb.append(p.val); first = false; }
        System.out.println(sb.toString());
    }
}
