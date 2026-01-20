// Rotate list right by k: find length L, make a ring, break at L-(k%L).
// Time O(n), Space O(1).
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node build(int[] vals) {
        Node dummy = new Node(0), cur = dummy;
        for (int v : vals) { cur.next = new Node(v); cur = cur.next; }
        return dummy.next;
    }

    static String toStr(Node head) {
        StringBuilder sb = new StringBuilder();
        while (head != null) {
            sb.append(head.val);
            if (head.next != null) sb.append(" -> ");
            head = head.next;
        }
        return sb.toString();
    }

    static Node rotateRight(Node head, int k) {
        if (head == null || head.next == null) return head;
        int L = 1;
        Node tail = head;
        while (tail.next != null) { tail = tail.next; L++; }
        k %= L;
        if (k == 0) return head;
        tail.next = head; // ring
        int steps = L - k;
        Node newTail = head;
        for (int i = 0; i < steps - 1; i++) newTail = newTail.next;
        Node newHead = newTail.next;
        newTail.next = null;
        return newHead;
    }

    public static void main(String[] args) {
        Node head = build(new int[]{1, 2, 3, 4, 5});
        System.out.println(toStr(rotateRight(head, 3)));
    }
}
