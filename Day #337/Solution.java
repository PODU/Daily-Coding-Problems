// Shuffle linked list uniformly via Fisher-Yates on node values.
// O(n) time, O(1) extra (in-place value swaps). Fixed seed -> deterministic.
import java.util.*;

public class Solution {
    static class Node { int val; Node next; Node(int v){val=v;} }

    public static void main(String[] args) {
        Node head = null, tail = null;
        for (int v = 1; v <= 5; ++v) {
            Node n = new Node(v);
            if (head == null) head = tail = n; else { tail.next = n; tail = n; }
        }
        List<Node> nodes = new ArrayList<>();
        for (Node p = head; p != null; p = p.next) nodes.add(p);
        int n = nodes.size();

        Random rng = new Random(42);
        for (int i = n - 1; i > 0; --i) {
            int j = rng.nextInt(i + 1);
            int t = nodes.get(i).val; nodes.get(i).val = nodes.get(j).val; nodes.get(j).val = t;
        }

        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) {
            if (sb.length() > 0) sb.append(' ');
            sb.append(p.val);
        }
        System.out.println(sb.toString());
    }
}
