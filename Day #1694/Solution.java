// Zigzag list: single pass, even index wants lst[i]<=lst[i+1], odd wants lst[i]>=lst[i+1]; swap if violated.
// O(n) time, O(1) extra space.
public class Solution {
    static class ListNode {
        int val;
        ListNode next;
        ListNode(int v) { val = v; }
    }

    static void zigzag(ListNode head) {
        ListNode cur = head;
        int i = 0;
        while (cur != null && cur.next != null) {
            if (i % 2 == 0) {
                if (cur.val > cur.next.val) { int t = cur.val; cur.val = cur.next.val; cur.next.val = t; }
            } else {
                if (cur.val < cur.next.val) { int t = cur.val; cur.val = cur.next.val; cur.next.val = t; }
            }
            cur = cur.next;
            i++;
        }
    }

    public static void main(String[] args) {
        ListNode head = new ListNode(1);
        ListNode cur = head;
        for (int v : new int[]{2, 3, 4, 5}) { cur.next = new ListNode(v); cur = cur.next; }

        zigzag(head);

        StringBuilder sb = new StringBuilder();
        for (cur = head; cur != null; cur = cur.next) {
            sb.append(cur.val);
            if (cur.next != null) sb.append(" -> ");
        }
        System.out.println(sb.toString());
    }
}
