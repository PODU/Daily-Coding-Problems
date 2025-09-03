// Day 208: Partition a linked list around pivot k (stable).
// Build two lists (< k and >= k) in original order, then splice. Time: O(n), Space: O(1).
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node partition(Node head, int k) {
        Node lessDummy = new Node(0), geDummy = new Node(0);
        Node less = lessDummy, ge = geDummy;
        for (Node cur = head; cur != null; cur = cur.next) {
            if (cur.val < k) { less.next = cur; less = cur; }
            else { ge.next = cur; ge = cur; }
        }
        ge.next = null;
        less.next = geDummy.next;
        return lessDummy.next;
    }

    static Node build(int[] v) {
        Node dummy = new Node(0), t = dummy;
        for (int x : v) { t.next = new Node(x); t = t.next; }
        return dummy.next;
    }

    static void print(Node h) {
        StringBuilder sb = new StringBuilder();
        for (; h != null; h = h.next) sb.append(h.val).append(h.next != null ? " -> " : "");
        System.out.println(sb);
    }

    public static void main(String[] args) {
        print(partition(build(new int[]{5, 1, 8, 0, 3}), 3)); // 1 -> 0 -> 5 -> 8 -> 3
    }
}
