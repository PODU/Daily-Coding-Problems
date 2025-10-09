// Remove k-th node from end in one pass via two pointers + dummy head. O(n) time, O(1) space.
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node removeKthFromEnd(Node head, int k) {
        Node dummy = new Node(0);
        dummy.next = head;
        Node fast = dummy, slow = dummy;
        for (int i = 0; i < k; i++) fast = fast.next; // advance fast k ahead
        while (fast.next != null) { fast = fast.next; slow = slow.next; }
        slow.next = slow.next.next;
        return dummy.next;
    }

    static void printList(Node head) {
        StringBuilder sb = new StringBuilder();
        for (Node c = head; c != null; c = c.next) {
            sb.append(c.val);
            if (c.next != null) sb.append(" -> ");
        }
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        head.next = new Node(2);
        head.next.next = new Node(3);
        head.next.next.next = new Node(4);
        head.next.next.next.next = new Node(5);
        head = removeKthFromEnd(head, 2);
        printList(head);
    }
}
