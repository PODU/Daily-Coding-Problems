// Uniform linked-list shuffle via Fisher-Yates. Time O(n), Space O(n) for the
// index pass (space-over-time variant: O(1) space, O(n^2) by random node selection).
// A deterministic LCG is used so output is reproducible.
public class Solution {
    static class Node { int val; Node next; Node(int v) { val = v; } }

    static long seed = 42;
    static long nextRand() {
        seed = (seed * 1103515245L + 12345L) % 2147483648L;
        return seed;
    }

    static Node shuffle(Node head) {
        java.util.List<Node> nodes = new java.util.ArrayList<>();
        for (Node p = head; p != null; p = p.next) nodes.add(p);
        int n = nodes.size();
        for (int i = n - 1; i >= 1; i--) {
            int j = (int) (nextRand() % (i + 1));
            int tmp = nodes.get(i).val;
            nodes.get(i).val = nodes.get(j).val;
            nodes.get(j).val = tmp;
        }
        return head;
    }

    public static void main(String[] args) {
        Node head = null, tail = null;
        for (int v = 1; v <= 5; v++) {
            Node node = new Node(v);
            if (head == null) { head = tail = node; } else { tail.next = node; tail = node; }
        }
        head = shuffle(head);
        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) {
            if (sb.length() > 0) sb.append(" -> ");
            sb.append(p.val);
        }
        System.out.println(sb);
    }
}
