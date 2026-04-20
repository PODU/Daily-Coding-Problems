// Reverse singly linked list in-place: iterative 3-pointer (prev/cur/next). O(n) time, O(1) space.
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node reverseList(Node head) {
        Node prev = null, cur = head;
        while (cur != null) {
            Node next = cur.next;
            cur.next = prev;
            prev = cur;
            cur = next;
        }
        return prev;
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        head.next = new Node(2);
        head.next.next = new Node(3);
        head.next.next.next = new Node(4);
        head.next.next.next.next = new Node(5);

        head = reverseList(head);

        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) {
            sb.append(p.val);
            if (p.next != null) sb.append(" -> ");
        }
        System.out.println(sb.toString());
    }
}
