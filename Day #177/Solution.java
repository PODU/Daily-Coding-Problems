// Rotate singly linked list right by k: form ring, break at (len - k%len). Time O(n), Space O(1).
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    public static Node rotateRight(Node head, int k) {
        if (head == null || head.next == null || k == 0) return head;
        int length = 1;
        Node tail = head;
        while (tail.next != null) { tail = tail.next; length++; }
        k %= length;
        if (k == 0) return head;
        tail.next = head; // make ring
        int steps = length - k;
        Node newTail = head;
        for (int i = 0; i < steps - 1; i++) newTail = newTail.next;
        Node newHead = newTail.next;
        newTail.next = null;
        return newHead;
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

    public static void main(String[] args) {
        System.out.println(toStr(rotateRight(build(new int[]{7, 7, 3, 5}), 2)));
        System.out.println(toStr(rotateRight(build(new int[]{1, 2, 3, 4, 5}), 3)));
    }
}
