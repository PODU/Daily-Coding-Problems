// Day 1143: Merge k sorted linked lists.
// Min-heap of list heads. Time O(N log k), Space O(k).
import java.util.*;

public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static Node mergeK(List<Node> lists) {
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

    static Node build(int[] v) {
        Node dummy = new Node(0), t = dummy;
        for (int x : v) { t.next = new Node(x); t = t.next; }
        return dummy.next;
    }

    public static void main(String[] args) {
        List<Node> lists = Arrays.asList(build(new int[]{1, 4, 7}),
                build(new int[]{2, 5, 8}), build(new int[]{3, 6, 9}));
        StringBuilder sb = new StringBuilder();
        for (Node n = mergeK(lists); n != null; n = n.next)
            sb.append(n.val).append(n.next != null ? " -> " : "");
        System.out.println(sb); // 1 -> 2 -> 3 -> 4 -> 5 -> 6 -> 7 -> 8 -> 9
    }
}
