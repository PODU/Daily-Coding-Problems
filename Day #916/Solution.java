// Reverse a singly linked list in-place by re-pointing each node's next pointer.
// Time: O(n), Space: O(1).
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node reverse(Node head) {
        Node prev = null;
        while (head != null) { Node nx = head.next; head.next = prev; prev = head; head = nx; }
        return prev;
    }

    static String toStr(Node h) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        while (h != null) { if (!first) sb.append(" -> "); sb.append(h.val); first = false; h = h.next; }
        return sb.toString();
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        head.next = new Node(2);
        head.next.next = new Node(3);
        head.next.next.next = new Node(4);
        head.next.next.next.next = new Node(5);
        head = reverse(head);
        System.out.println(toStr(head)); // 5 -> 4 -> 3 -> 2 -> 1
    }
}
