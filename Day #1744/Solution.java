// Merge k sorted linked lists via min-heap of current heads. O(N log k) time, O(k) space.
import java.util.PriorityQueue;

public class Solution {
    static class Node {
        int val;
        Node next;
        Node(int v) { val = v; }
    }

    static Node build(int[] v) {
        Node dummy = new Node(0), t = dummy;
        for (int x : v) { t.next = new Node(x); t = t.next; }
        return dummy.next;
    }

    static Node mergeK(Node[] lists) {
        PriorityQueue<Node> pq = new PriorityQueue<>((a, b) -> a.val - b.val);
        for (Node l : lists) if (l != null) pq.add(l);
        Node dummy = new Node(0), tail = dummy;
        while (!pq.isEmpty()) {
            Node n = pq.poll();
            tail.next = n; tail = n;
            if (n.next != null) pq.add(n.next);
        }
        return dummy.next;
    }

    public static void main(String[] args) {
        Node[] lists = { build(new int[]{1,4,5}), build(new int[]{1,3,4}), build(new int[]{2,6}) };
        Node m = mergeK(lists);
        StringBuilder sb = new StringBuilder();
        for (Node p = m; p != null; p = p.next) {
            if (sb.length() > 0) sb.append(" ");
            sb.append(p.val);
        }
        System.out.println(sb.toString());
    }
}
