// Day 127: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. O(max(m,n)) time, O(1) extra space.
public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node addLists(Node a, Node b) {
        Node dummy = new Node(0), tail = dummy;
        int carry = 0;
        while (a != null || b != null || carry != 0) {
            int s = carry;
            if (a != null) { s += a.val; a = a.next; }
            if (b != null) { s += b.val; b = b.next; }
            carry = s / 10;
            tail.next = new Node(s % 10);
            tail = tail.next;
        }
        return dummy.next;
    }

    static Node build(int[] d) {
        Node dummy = new Node(0), t = dummy;
        for (int v : d) { t.next = new Node(v); t = t.next; }
        return dummy.next;
    }

    static void print(Node n) {
        StringBuilder sb = new StringBuilder();
        while (n != null) {
            sb.append(n.val);
            if (n.next != null) sb.append(" -> ");
            n = n.next;
        }
        System.out.println(sb);
    }

    public static void main(String[] args) {
        Node a = build(new int[]{9, 9}); // 99
        Node b = build(new int[]{5, 2}); // 25
        print(addLists(a, b));           // 4 -> 2 -> 1
    }
}
