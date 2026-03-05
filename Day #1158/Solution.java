// Bottom-up iterative merge sort on a singly linked list. O(n log n) time, O(1) auxiliary space.
public class Solution {
    static class ListNode {
        int val;
        ListNode next;
        ListNode(int v) { val = v; }
    }

    // Take `n` nodes from head, cut, return remainder.
    static ListNode split(ListNode head, int n) {
        for (int i = 1; head != null && i < n; i++) head = head.next;
        if (head == null) return null;
        ListNode second = head.next;
        head.next = null;
        return second;
    }

    // Merge sorted lists a, b onto tail; return new tail.
    static ListNode merge(ListNode a, ListNode b, ListNode tail) {
        ListNode cur = tail;
        while (a != null && b != null) {
            if (a.val <= b.val) { cur.next = a; a = a.next; }
            else { cur.next = b; b = b.next; }
            cur = cur.next;
        }
        cur.next = (a != null) ? a : b;
        while (cur.next != null) cur = cur.next;
        return cur;
    }

    static ListNode sortList(ListNode head) {
        if (head == null || head.next == null) return head;
        int n = 0;
        for (ListNode p = head; p != null; p = p.next) n++;

        ListNode dummy = new ListNode(0);
        dummy.next = head;
        for (int size = 1; size < n; size <<= 1) {
            ListNode cur = dummy.next;
            ListNode tail = dummy;
            while (cur != null) {
                ListNode left = cur;
                ListNode right = split(left, size);
                cur = split(right, size);
                tail = merge(left, right, tail);
            }
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        int[] vals = {4, 1, -3, 99};
        ListNode head = null, tail = null;
        for (int v : vals) {
            ListNode node = new ListNode(v);
            if (head == null) { head = tail = node; }
            else { tail.next = node; tail = node; }
        }

        head = sortList(head);

        StringBuilder sb = new StringBuilder();
        boolean first = true;
        for (ListNode p = head; p != null; p = p.next) {
            if (!first) sb.append(" -> ");
            sb.append(p.val);
            first = false;
        }
        System.out.println(sb.toString());
    }
}
