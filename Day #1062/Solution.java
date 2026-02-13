// Day 1062: Swap every two adjacent nodes in a singly linked list.
// Approach: iterative pointer manipulation. Time O(n), Space O(1).
public class Solution {
    static class Node {
        int val;
        Node next;
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

    static void printList(Node head) {
        StringBuilder sb = new StringBuilder();
        while (head != null) {
            if (sb.length() > 0) sb.append(" -> ");
            sb.append(head.val);
            head = head.next;
        }
        System.out.println(sb.toString());
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        head.next = new Node(2);
        head.next.next = new Node(3);
        head.next.next.next = new Node(4);
        head = swapPairs(head);
        printList(head); // 2 -> 1 -> 4 -> 3
    }
}
