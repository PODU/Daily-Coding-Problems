// Zigzag rearrange linked list values in a single pass by swapping adjacent
// node values that violate the expected ordering. Time O(n), Space O(1).
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static void zigzag(Node head) {
        boolean less = true; // even index expects list[i] <= list[i+1]
        for (Node cur = head; cur != null && cur.next != null; cur = cur.next) {
            if (less) {
                if (cur.val > cur.next.val) { int t = cur.val; cur.val = cur.next.val; cur.next.val = t; }
            } else {
                if (cur.val < cur.next.val) { int t = cur.val; cur.val = cur.next.val; cur.next.val = t; }
            }
            less = !less;
        }
    }

    public static void main(String[] args) {
        int[] vals = {1, 2, 3, 4, 5};
        Node head = null, tail = null;
        for (int v : vals) {
            Node n = new Node(v);
            if (head == null) head = tail = n;
            else { tail.next = n; tail = n; }
        }
        zigzag(head);
        StringBuilder sb = new StringBuilder();
        for (Node cur = head; cur != null; cur = cur.next) {
            sb.append(cur.val);
            if (cur.next != null) sb.append(" -> ");
        }
        System.out.println(sb.toString());
    }
}
