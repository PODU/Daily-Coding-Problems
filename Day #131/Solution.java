// Day 131: Deep clone a linked list with next + random pointers.
// Interleaving trick (weave copies, set randoms, unweave). O(n) time, O(1) extra space.
public class Solution {
    static class Node {
        int val;
        Node next, random;
        Node(int v) { val = v; }
    }

    static Node clone(Node head) {
        if (head == null) return null;
        for (Node c = head; c != null; c = c.next.next) {
            Node cp = new Node(c.val);
            cp.next = c.next;
            c.next = cp;
        }
        for (Node c = head; c != null; c = c.next.next)
            if (c.random != null) c.next.random = c.random.next;
        Node newHead = head.next;
        for (Node c = head; c != null; c = c.next) {
            Node cp = c.next;
            c.next = cp.next;
            if (cp.next != null) cp.next = cp.next.next;
        }
        return newHead;
    }

    public static void main(String[] args) {
        Node[] n = new Node[5];
        for (int v = 0; v < 5; v++) n[v] = new Node(v + 1);
        for (int i = 0; i < 4; i++) n[i].next = n[i + 1];
        n[0].random = n[2];
        n[1].random = n[0];
        n[2].random = n[4];
        n[3].random = n[1];
        n[4].random = n[4];

        Node copy = clone(n[0]);
        boolean separate = true;
        Node o = n[0], c = copy;
        while (c != null) {
            if (c == o) separate = false;
            System.out.println("node " + c.val + " -> random " + (c.random != null ? c.random.val : 0));
            o = o.next;
            c = c.next;
        }
        System.out.println("deep copy verified: " + separate);
    }
}
