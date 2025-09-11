// Day 256: Rearrange linked list values into zigzag low->high->low form.
// One pass over node values: at even i ensure a[i]<=a[i+1], at odd i ensure a[i]>=a[i+1], swap on violation.
// Time: O(n), Space: O(1).
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static void wiggle(Node head) {
        boolean less = true; // even index: want current <= next
        for (Node cur = head; cur != null && cur.next != null; cur = cur.next) {
            if (less) {
                if (cur.val > cur.next.val) { int t = cur.val; cur.val = cur.next.val; cur.next.val = t; }
            } else {
                if (cur.val < cur.next.val) { int t = cur.val; cur.val = cur.next.val; cur.next.val = t; }
            }
            less = !less;
        }
    }

    static String listToString(Node head) {
        StringBuilder sb = new StringBuilder();
        for (Node cur = head; cur != null; cur = cur.next) {
            sb.append(cur.val);
            if (cur.next != null) sb.append(" -> ");
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        int[] vals = {1, 2, 3, 4, 5};
        Node head = null, tail = null;
        for (int v : vals) {
            Node n = new Node(v);
            if (head == null) { head = tail = n; }
            else { tail.next = n; tail = n; }
        }
        wiggle(head);
        System.out.println(listToString(head)); // 1 -> 3 -> 2 -> 5 -> 4
    }
}
