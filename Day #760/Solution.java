// Day 760: Uniformly shuffle a linked list. Space-prioritized variant:
// forward Fisher-Yates that swaps node values in place using O(1) extra space
// at the cost of O(n^2) time (re-walks to pick a uniform remaining node).
// A deterministic LCG is used so the demo output is reproducible.
public class Solution {
    static class Node {
        int val; Node next;
        Node(int v) { val = v; }
    }

    static class LCG {
        long s;
        LCG(long seed) { s = seed; }
        int next() { s = (s * 1103515245L + 12345L) & 0x7fffffffL; return (int) s; }
    }

    static void shuffle(Node head, LCG rng) {
        for (Node p = head; p != null; p = p.next) {
            int m = 0;
            for (Node t = p; t != null; t = t.next) m++;
            int r = rng.next() % m;
            Node q = p;
            while (r-- > 0) q = q.next;
            int tmp = p.val; p.val = q.val; q.val = tmp;
        }
    }

    static void printList(Node head) {
        StringBuilder sb = new StringBuilder();
        for (Node p = head; p != null; p = p.next) {
            sb.append(p.val);
            if (p.next != null) sb.append(" -> ");
        }
        System.out.println(sb);
    }

    public static void main(String[] args) {
        Node head = new Node(1);
        Node cur = head;
        for (int v = 2; v <= 5; v++) { cur.next = new Node(v); cur = cur.next; }

        System.out.print("original: "); printList(head);
        shuffle(head, new LCG(42));
        System.out.print("shuffled: "); printList(head);
    }
}
