// Remove all consecutive nodes summing to zero using prefix sums + hash map.
// A prefix sum seen before means the span between is zero-sum and is excised.
// Time O(n), Space O(n).
import java.util.*;

public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node removeZeroSum(Node head) {
        Node dummy = new Node(0);
        dummy.next = head;
        Map<Integer, Node> seen = new HashMap<>();
        int sum = 0;
        for (Node p = dummy; p != null; p = p.next) {
            sum += p.val;
            seen.put(sum, p);
        }
        sum = 0;
        for (Node p = dummy; p != null; p = p.next) {
            sum += p.val;
            p.next = seen.get(sum).next;
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        int[] vals = {3, 4, -7, 5, -6, 6};
        Node head = null, tail = null;
        for (int v : vals) {
            Node n = new Node(v);
            if (head == null) head = tail = n;
            else { tail.next = n; tail = n; }
        }
        head = removeZeroSum(head);
        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) {
            if (sb.length() > 0) sb.append(' ');
            sb.append(p.val);
        }
        System.out.println(sb);
    }
}
