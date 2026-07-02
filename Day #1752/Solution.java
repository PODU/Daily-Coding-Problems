// Day 1752: Remove kth-from-last node of a singly linked list in ONE pass, O(1) space.
// Two pointers spaced k apart; when fast reaches end, slow is just before the target. O(n) time.
// Assumption (no README example): list 1->2->3->4->5, k=2 removes value 4 -> "1 2 3 5".
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node removeKthLast(Node head, int k) {
        Node dummy = new Node(0);
        dummy.next = head;
        Node fast = dummy, slow = dummy;
        for (int i = 0; i < k; i++) fast = fast.next; // advance fast k steps
        while (fast.next != null) { fast = fast.next; slow = slow.next; }
        slow.next = slow.next.next; // unlink target
        return dummy.next;
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        head.next = new Node(2);
        head.next.next = new Node(3);
        head.next.next.next = new Node(4);
        head.next.next.next.next = new Node(5);

        head = removeKthLast(head, 2);
        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) {
            sb.append(p.val);
            if (p.next != null) sb.append(' ');
        }
        System.out.println(sb); // 1 2 3 5
    }
}
