// Day 1025: Remove all consecutive linked-list nodes that sum to zero.
// Approach: prefix-sum + hashmap of last node per prefix sum (dummy head). O(N) time, O(N) space.
import java.util.HashMap;
import java.util.Map;

public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node removeZeroSum(Node head) {
        Node dummy = new Node(0);
        dummy.next = head;
        Map<Integer, Node> last = new HashMap<>();
        int sum = 0;
        for (Node cur = dummy; cur != null; cur = cur.next) {
            sum += cur.val;
            last.put(sum, cur); // keep last occurrence
        }
        sum = 0;
        for (Node cur = dummy; cur != null; cur = cur.next) {
            sum += cur.val;
            cur.next = last.get(sum).next;
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
        StringBuilder out = new StringBuilder();
        for (Node c = head; c != null; c = c.next) {
            if (out.length() > 0) out.append(" -> ");
            out.append(c.val);
        }
        System.out.println(out.toString());
    }
}
