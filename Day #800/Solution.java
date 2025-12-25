// Day 800: Rearrange list values into low->high->low... (wiggle).
// One pass: at even idx ensure a<=next, at odd idx ensure a>=next; swap if not.
// Time O(N), Space O(1).
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static void wiggle(Node head) {
        boolean less = true;
        for (Node cur = head; cur != null && cur.next != null; cur = cur.next) {
            if (less ? (cur.val > cur.next.val) : (cur.val < cur.next.val)) {
                int t = cur.val; cur.val = cur.next.val; cur.next.val = t;
            }
            less = !less;
        }
    }

    public static void main(String[] args) {
        Node head = new Node(1), c = head;
        for (int v : new int[]{2, 3, 4, 5}) { c.next = new Node(v); c = c.next; }
        wiggle(head);
        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next)
            sb.append(p.val).append(p.next != null ? " -> " : "");
        System.out.println(sb); // 1 -> 3 -> 2 -> 5 -> 4
    }
}
