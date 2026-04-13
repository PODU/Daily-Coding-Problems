// Remove consecutive nodes summing to zero: dummy head, prefix-sum -> last node map;
// repeated prefix means a zero-sum span to splice out. Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static Node removeZeroSum(Node head) {
        Node dummy = new Node(0);
        dummy.next = head;
        Map<Integer, Node> seen = new HashMap<>();
        int prefix = 0;
        for (Node cur = dummy; cur != null; cur = cur.next) {
            prefix += cur.val;
            seen.put(prefix, cur); // last node achieving this prefix sum
        }
        prefix = 0;
        for (Node cur = dummy; cur != null; cur = cur.next) {
            prefix += cur.val;
            cur.next = seen.get(prefix).next; // skip zero-sum span
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        int[] vals = {3, 4, -7, 5, -6, 6};
        Node head = null, tail = null;
        for (int v : vals) {
            Node n = new Node(v);
            if (head == null) { head = tail = n; }
            else { tail.next = n; tail = n; }
        }
        head = removeZeroSum(head);
        StringBuilder sb = new StringBuilder();
        for (Node c = head; c != null; c = c.next) {
            if (sb.length() > 0) sb.append(" -> ");
            sb.append(c.val);
        }
        System.out.println(sb.toString());
    }
}
