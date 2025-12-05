// Day 699: Rotate a singly linked list right by k places.
// Approach: close into a ring, then break it (len - k%len) nodes ahead.
// Time O(n), Space O(1).
public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static Node rotateRight(Node head, int k) {
        if (head == null || head.next == null) return head;
        int len = 1; Node tail = head;
        while (tail.next != null) { tail = tail.next; len++; }
        k %= len; if (k == 0) return head;
        tail.next = head;                    // ring
        int stepsToNewTail = len - k;
        Node newTail = head;
        for (int i = 1; i < stepsToNewTail; i++) newTail = newTail.next;
        Node newHead = newTail.next;
        newTail.next = null;
        return newHead;
    }

    static Node build(int[] v) { Node d = new Node(0), c = d; for (int x : v) { c.next = new Node(x); c = c.next; } return d.next; }
    static void print(Node h) { StringBuilder sb = new StringBuilder(); for (; h != null; h = h.next) { sb.append(h.val); if (h.next != null) sb.append(" -> "); } System.out.println(sb); }

    public static void main(String[] args) {
        print(rotateRight(build(new int[]{7, 7, 3, 5}), 2));    // 3 -> 5 -> 7 -> 7
        print(rotateRight(build(new int[]{1, 2, 3, 4, 5}), 3)); // 3 -> 4 -> 5 -> 1 -> 2
    }
}
