// Day 417: Remove all consecutive nodes summing to zero via prefix-sum + hashmap.
// Time O(n), Space O(n).
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static class ListNode {
        int val;
        ListNode next;
        ListNode(int v) { val = v; }
    }

    static ListNode removeZeroSum(ListNode head) {
        ListNode dummy = new ListNode(0);
        dummy.next = head;
        Map<Integer, ListNode> seen = new HashMap<>();
        int prefix = 0;
        for (ListNode node = dummy; node != null; node = node.next) {
            prefix += node.val;
            seen.put(prefix, node); // keep latest node for this prefix sum
        }
        prefix = 0;
        for (ListNode node = dummy; node != null; node = node.next) {
            prefix += node.val;
            node.next = seen.get(prefix).next; // skip zero-sum run
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        int[] vals = {3, 4, -7, 5, -6, 6};
        ListNode dummy = new ListNode(0);
        ListNode tail = dummy;
        for (int v : vals) { tail.next = new ListNode(v); tail = tail.next; }
        ListNode head = removeZeroSum(dummy.next);
        StringBuilder sb = new StringBuilder();
        for (ListNode n = head; n != null; n = n.next) {
            if (sb.length() > 0) sb.append(" -> ");
            sb.append(n.val);
        }
        System.out.println(sb.toString());
    }
}
