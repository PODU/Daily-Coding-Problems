// Day 1432: Deep clone a linked list with a random pointer.
// Approach: interleave cloned nodes, wire randoms, then split. Time: O(n), Space: O(1) extra.
public class Solution {
    static class Node {
        int val;
        Node next, random;
        Node(int v) { val = v; }
    }

    static Node cloneList(Node head) {
        if (head == null) return null;
        for (Node cur = head; cur != null; cur = cur.next.next) {
            Node copy = new Node(cur.val);
            copy.next = cur.next;
            cur.next = copy;
        }
        for (Node cur = head; cur != null; cur = cur.next.next) {
            if (cur.random != null) cur.next.random = cur.random.next;
        }
        Node newHead = head.next;
        for (Node cur = head; cur != null; cur = cur.next) {
            Node copy = cur.next;
            cur.next = copy.next;
            if (copy.next != null) copy.next = copy.next.next;
        }
        return newHead;
    }

    public static void main(String[] args) {
        Node a = new Node(1), b = new Node(2), c = new Node(3);
        a.next = b; b.next = c;
        a.random = c; b.random = a; c.random = c;

        Node cloned = cloneList(a);
        boolean ok = true;
        Node p = a, q = cloned;
        while (p != null) {
            if (q == p) ok = false;
            if (q.val != p.val) ok = false;
            if ((p.random == null) != (q.random == null)) ok = false;
            if (p.random != null && q.random.val != p.random.val) ok = false;
            p = p.next; q = q.next;
        }
        System.out.println(ok
            ? "Clone verified: values and random targets match, nodes distinct"
            : "Clone FAILED");
    }
}
