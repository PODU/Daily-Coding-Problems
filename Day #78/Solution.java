// Day 78: Merge k sorted linked lists using a min-heap.
// Time O(N log k) where N total nodes, Space O(k).
import java.util.*;

public class Solution {
    static class ListNode {
        int val; ListNode next;
        ListNode(int v) { val = v; }
    }

    static ListNode mergeKLists(List<ListNode> lists) {
        PriorityQueue<ListNode> pq = new PriorityQueue<>((a, b) -> Integer.compare(a.val, b.val));
        for (ListNode l : lists) if (l != null) pq.add(l);
        ListNode dummy = new ListNode(0), tail = dummy;
        while (!pq.isEmpty()) {
            ListNode n = pq.poll();
            tail.next = n; tail = n;
            if (n.next != null) pq.add(n.next);
        }
        return dummy.next;
    }

    static ListNode build(int[] v) {
        ListNode dummy = new ListNode(0), t = dummy;
        for (int x : v) { t.next = new ListNode(x); t = t.next; }
        return dummy.next;
    }

    public static void main(String[] args) {
        List<ListNode> lists = Arrays.asList(
            build(new int[]{1,4,5}), build(new int[]{1,3,4}), build(new int[]{2,6}));
        ListNode m = mergeKLists(lists);
        StringBuilder sb = new StringBuilder();
        for (; m != null; m = m.next) sb.append(m.val).append(m.next != null ? " -> " : "");
        System.out.println(sb); // 1 -> 1 -> 2 -> 3 -> 4 -> 4 -> 5 -> 6
    }
}
