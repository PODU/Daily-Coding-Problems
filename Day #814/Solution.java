// Add two numbers stored as reversed-digit linked lists via elementary addition
// with carry, walking both lists. Time O(max(m,n)), Space O(max(m,n)).
public class Solution {
    static class ListNode {
        int val; ListNode next;
        ListNode(int v) { val = v; }
    }

    static ListNode build(int[] d) {
        ListNode dummy = new ListNode(0), cur = dummy;
        for (int x : d) { cur.next = new ListNode(x); cur = cur.next; }
        return dummy.next;
    }

    static ListNode addLists(ListNode a, ListNode b) {
        ListNode dummy = new ListNode(0), cur = dummy;
        int carry = 0;
        while (a != null || b != null || carry != 0) {
            int s = carry + (a != null ? a.val : 0) + (b != null ? b.val : 0);
            carry = s / 10;
            cur.next = new ListNode(s % 10);
            cur = cur.next;
            if (a != null) a = a.next;
            if (b != null) b = b.next;
        }
        return dummy.next;
    }

    static String toStr(ListNode n) {
        StringBuilder sb = new StringBuilder();
        boolean first = true;
        while (n != null) {
            if (!first) sb.append(" -> ");
            sb.append(n.val); first = false; n = n.next;
        }
        return sb.toString();
    }

    public static void main(String[] args) {
        ListNode a = build(new int[]{9, 9});
        ListNode b = build(new int[]{5, 2});
        System.out.println(toStr(addLists(a, b)));
    }
}
