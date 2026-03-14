// Day 1207: Remove kth-from-last node in one pass, constant space.
// Two pointers k apart; when lead hits end, trail is just before target. Time O(n), Space O(1).
public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static Node removeKthLast(Node head, int k) {
        Node dummy = new Node(0); dummy.next = head;
        Node lead = dummy, trail = dummy;
        for (int i = 0; i < k; i++) lead = lead.next;
        while (lead.next != null) { lead = lead.next; trail = trail.next; }
        trail.next = trail.next.next;
        return dummy.next;
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        head.next = new Node(2); head.next.next = new Node(3);
        head.next.next.next = new Node(4); head.next.next.next.next = new Node(5);
        head = removeKthLast(head, 2); // remove 4
        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) { if (sb.length() > 0) sb.append(" -> "); sb.append(p.val); }
        System.out.println(sb); // 1 -> 2 -> 3 -> 5
    }
}
