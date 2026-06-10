// Swap every two adjacent nodes in a singly linked list via iterative pointer swaps.
// Time: O(n), Space: O(1).
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

    public static void main(String[] args) {
        Node head = new Node(1);
        head.next = new Node(2);
        head.next.next = new Node(3);
        head.next.next.next = new Node(4);

        head = swapPairs(head);

        StringBuilder sb = new StringBuilder();
        Node cur = head;
        boolean first = true;
        while (cur != null) {
            if (!first) sb.append(" -> ");
            sb.append(cur.val);
            first = false;
            cur = cur.next;
        }
        System.out.println(sb.toString());
    }
}
