// Rearrange linked list values to low->high->low->high. One pass swapping
// adjacent values to enforce the alternating relation. Time O(n), Space O(1).
public class Solution {
    static class Node { int val; Node next; Node(int v){ val = v; } }

    static void zigzag(Node head) {
        boolean low = true; // current pair should satisfy a <= b
        for (Node c = head; c != null && c.next != null; c = c.next, low = !low) {
            if (low ? (c.val > c.next.val) : (c.val < c.next.val)) {
                int t = c.val; c.val = c.next.val; c.next.val = t;
            }
        }
    }

    public static void main(String[] args) {
        Node head = null, tail = null;
        for (int v : new int[]{1,2,3,4,5}) {
            Node n = new Node(v);
            if (head == null) head = tail = n; else { tail.next = n; tail = n; }
        }
        zigzag(head);
        StringBuilder sb = new StringBuilder();
        for (Node c = head; c != null; c = c.next) {
            sb.append(c.val);
            if (c.next != null) sb.append(" -> ");
        }
        System.out.println(sb); // 1 -> 3 -> 2 -> 5 -> 4
    }
}
