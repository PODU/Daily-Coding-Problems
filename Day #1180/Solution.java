// Day 1180: Swap every two adjacent nodes in a singly linked list.
// Iterative pointer rewiring with a dummy head. Time O(N), Space O(1).
public class Solution {
    static class ListNode {
        int val; ListNode next;
        ListNode(int v) { val = v; }
    }

    static ListNode swapPairs(ListNode head) {
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        ListNode prev = dummy;
        while (prev.next != null && prev.next.next != null) {
            ListNode a = prev.next, b = a.next;
            a.next = b.next;
            b.next = a;
            prev.next = b;
            prev = a;
        }
        return dummy.next;
    }

    static ListNode build(int[] v) {
        ListNode dummy = new ListNode(0), t = dummy;
        for (int x : v) { t.next = new ListNode(x); t = t.next; }
        return dummy.next;
    }

    public static void main(String[] args) {
        ListNode h = swapPairs(build(new int[]{1, 2, 3, 4}));
        StringBuilder sb = new StringBuilder();
        for (; h != null; h = h.next) sb.append(h.val).append(h.next != null ? " -> " : "");
        System.out.println(sb); // 2 -> 1 -> 4 -> 3
    }
}
