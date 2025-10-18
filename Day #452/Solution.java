// Day 452: Add two numbers stored as reversed-digit linked lists.
// Single pass with carry. Time O(max(n,m)), Space O(max(n,m)).
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
            int sum = carry;
            if (a != null) { sum += a.val; a = a.next; }
            if (b != null) { sum += b.val; b = b.next; }
            carry = sum / 10;
            tail.next = new Node(sum % 10);
            tail = tail.next;
        }
        return dummy.next;
    }

    static Node build(int... xs) {
        Node dummy = new Node(0), t = dummy;
        for (int x : xs) { t.next = new Node(x); t = t.next; }
        return dummy.next;
    }

    static void print(Node n) {
        StringBuilder sb = new StringBuilder();
        while (n != null) {
            if (sb.length() > 0) sb.append(" -> ");
            sb.append(n.val);
            n = n.next;
        }
        System.out.println(sb);
    }

    public static void main(String[] args) {
        print(addLists(build(9, 9), build(5, 2))); // 4 -> 2 -> 1
    }
}
