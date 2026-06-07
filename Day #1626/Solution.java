// Day 1626: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(m,n)), Space O(max(m,n)).
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node addLists(Node a, Node b) {
        Node dummy = new Node(0), tail = dummy;
        int carry = 0;
        while (a != null || b != null || carry != 0) {
            int sum = carry;
            if (a != null) { sum += a.val; a = a.next; }
            if (b != null) { sum += b.val; b = b.next; }
            carry = sum / 10;
            tail.next = new Node(sum % 10);
            tail = tail.next;
        }
        return dummy.next;
    }

    static Node build(int[] v) {
        Node dummy = new Node(0), t = dummy;
        for (int x : v) { t.next = new Node(x); t = t.next; }
        return dummy.next;
    }

    public static void main(String[] args) {
        Node r = addLists(build(new int[]{9, 9}), build(new int[]{5, 2}));
        StringBuilder sb = new StringBuilder();
        for (Node c = r; c != null; c = c.next) sb.append(c.val).append(c.next != null ? " -> " : "");
        System.out.println(sb);
    }
}
