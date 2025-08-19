// Swap every two adjacent nodes in a singly linked list via iterative pointer rewiring. O(n) time, O(1) space.

public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node swapPairs(Node head) {
        Node dummy = new Node(0);
        dummy.next = head;
        Node prev = dummy;
        while (prev.next != null && prev.next.next != null) {
            Node a = prev.next;
            Node b = a.next;
            a.next = b.next;
            b.next = a;
            prev.next = b;
            prev = a;
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        head.next = new Node(2);
        head.next.next = new Node(3);
        head.next.next.next = new Node(4);
        head = swapPairs(head);
        StringBuilder sb = new StringBuilder();
        for (Node c = head; c != null; c = c.next) {
            sb.append(c.val);
            if (c.next != null) sb.append(" -> ");
        }
        System.out.println(sb); // 2 -> 1 -> 4 -> 3
    }
}
