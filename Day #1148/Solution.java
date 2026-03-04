// Day 1148: Rotate linked list right by k.
// Find length, close into ring, cut at (len - k%len). O(n) time, O(1) space.
public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static Node rotate(Node head, int k) {
        if (head == null || head.next == null) return head;
        int len = 1;
        Node tail = head;
        while (tail.next != null) { tail = tail.next; len++; }
        k %= len;
        if (k == 0) return head;
        tail.next = head;
        int steps = len - k;
        Node newTail = head;
        for (int i = 1; i < steps; i++) newTail = newTail.next;
        Node newHead = newTail.next;
        newTail.next = null;
        return newHead;
    }

    static Node build(int[] v) {
        Node dummy = new Node(0), t = dummy;
        for (int x : v) { t.next = new Node(x); t = t.next; }
        return dummy.next;
    }

    static String toStr(Node h) {
        StringBuilder sb = new StringBuilder();
        for (; h != null; h = h.next) sb.append(h.val).append(h.next != null ? " -> " : "");
        return sb.toString();
    }

    public static void main(String[] args) {
        System.out.println(toStr(rotate(build(new int[]{7, 7, 3, 5}), 2)));    // 3 -> 5 -> 7 -> 7
        System.out.println(toStr(rotate(build(new int[]{1, 2, 3, 4, 5}), 3))); // 3 -> 4 -> 5 -> 1 -> 2
    }
}
