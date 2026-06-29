// Iterative in-place reversal of a singly linked list using three pointers.
// Time: O(n), Space: O(1).
public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static Node reverse(Node head) {
        Node prev = null;
        while (head != null) {
            Node nxt = head.next;
            head.next = prev;
            prev = head;
            head = nxt;
        }
        return prev;
    }

    public static void main(String[] args) {
        Node head = null;
        for (int i = 5; i >= 1; i--) { Node n = new Node(i); n.next = head; head = n; }
        head = reverse(head);
        StringBuilder sb = new StringBuilder();
        for (Node c = head; c != null; c = c.next) {
            sb.append(c.val);
            if (c.next != null) sb.append(' ');
        }
        System.out.println(sb.toString());
    }
}
