// Uniformly shuffle a linked list via Fisher-Yates on node values. Time O(n), Space O(n).
// Space-over-time alternative: walk to a random remaining node and swap in place -> O(1) extra, O(n^2) time.
import java.util.*;

public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static Node build(int[] arr) {
        Node head = null, tail = null;
        for (int x : arr) { Node n = new Node(x); if (head == null) { head = tail = n; } else { tail.next = n; tail = n; } }
        return head;
    }
    static List<Integer> toList(Node h) {
        List<Integer> a = new ArrayList<>();
        for (Node c = h; c != null; c = c.next) a.add(c.val);
        return a;
    }
    static void shuffleList(Node head, Random rng) {
        List<Node> nodes = new ArrayList<>();
        for (Node c = head; c != null; c = c.next) nodes.add(c);
        for (int i = nodes.size() - 1; i > 0; i--) {
            int j = rng.nextInt(i + 1);
            int t = nodes.get(i).val; nodes.get(i).val = nodes.get(j).val; nodes.get(j).val = t;
        }
    }

    public static void main(String[] args) {
        Node head = build(new int[]{1, 2, 3, 4, 5});
        System.out.println("Original: " + toList(head));
        shuffleList(head, new Random(42));
        System.out.println("Shuffled: " + toList(head));
    }
}
