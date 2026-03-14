// Day 1205: Add two numbers stored as reversed-digit linked lists.
// Traverse both lists with a running carry. Time O(max(m,n)), Space O(max(m,n)).
public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static Node build(int[] ds) {
        Node dummy = new Node(0), t = dummy;
        for (int d : ds) { t.next = new Node(d); t = t.next; }
        return dummy.next;
    }

    static Node addLists(Node a, Node b) {
        Node dummy = new Node(0), t = dummy;
        int carry = 0;
        while (a != null || b != null || carry != 0) {
            int s = carry + (a != null ? a.val : 0) + (b != null ? b.val : 0);
            carry = s / 10;
            t.next = new Node(s % 10); t = t.next;
            if (a != null) a = a.next;
            if (b != null) b = b.next;
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        Node s = addLists(build(new int[]{9, 9}), build(new int[]{5, 2}));
        StringBuilder sb = new StringBuilder();
        for (Node p = s; p != null; p = p.next) { if (sb.length() > 0) sb.append(" -> "); sb.append(p.val); }
        System.out.println(sb); // 4 -> 2 -> 1
    }
}
